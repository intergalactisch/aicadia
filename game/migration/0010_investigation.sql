ALTER TABLE activity
    DROP CONSTRAINT activity_operation_check,
    DROP CONSTRAINT activity_confirmed_mutation_provenance_check,
    ADD CONSTRAINT activity_operation_check CHECK (
        operation IN (
            'create_character',
            'create_entity',
            'create_entry_place',
            'enter_world',
            'submit_action',
            'submit_interaction',
            'submit_discovery'
        )
    ),
    ADD CONSTRAINT activity_confirmed_mutation_provenance_check CHECK (
        (
            operation IN ('submit_action', 'submit_interaction', 'submit_discovery')
            AND prose IS NOT NULL
            AND request_id IS NOT NULL
            AND request_fingerprint IS NOT NULL
        ) OR (
            operation NOT IN ('submit_action', 'submit_interaction', 'submit_discovery')
            AND prose IS NULL
            AND request_id IS NULL
            AND request_fingerprint IS NULL
        )
    );

CREATE TABLE investigation_attempt (
    id uuid NOT NULL,
    requested_by_user_id uuid NOT NULL,
    request_id uuid NOT NULL,
    character_entity_id uuid NOT NULL,
    place_entity_id uuid NOT NULL,
    outcome text NOT NULL,
    consumed_by_activity_id uuid,
    voided_by_attempt_id uuid,
    created_at timestamptz NOT NULL,
    CONSTRAINT investigation_attempt_pkey PRIMARY KEY (id),
    CONSTRAINT investigation_attempt_requested_by_user_id_request_id_key
        UNIQUE (requested_by_user_id, request_id),
    CONSTRAINT investigation_attempt_requested_by_user_id_fkey
        FOREIGN KEY (requested_by_user_id) REFERENCES "user"(id) ON DELETE RESTRICT,
    CONSTRAINT investigation_attempt_character_entity_id_fkey
        FOREIGN KEY (character_entity_id) REFERENCES character(entity_id) ON DELETE RESTRICT,
    CONSTRAINT investigation_attempt_place_entity_id_fkey
        FOREIGN KEY (place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT,
    CONSTRAINT investigation_attempt_consumed_by_activity_id_key
        UNIQUE (consumed_by_activity_id),
    CONSTRAINT investigation_attempt_consumed_by_activity_id_fkey
        FOREIGN KEY (consumed_by_activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT investigation_attempt_voided_by_attempt_id_fkey
        FOREIGN KEY (voided_by_attempt_id) REFERENCES investigation_attempt(id) ON DELETE RESTRICT,
    CONSTRAINT investigation_attempt_outcome_check CHECK (outcome IN ('zero', 'positive')),
    CONSTRAINT investigation_attempt_zero_lifecycle_check CHECK (
        outcome = 'positive'
        OR (consumed_by_activity_id IS NULL AND voided_by_attempt_id IS NULL)
    ),
    CONSTRAINT investigation_attempt_one_lifecycle_check CHECK (
        consumed_by_activity_id IS NULL OR voided_by_attempt_id IS NULL
    ),
    CONSTRAINT investigation_attempt_no_self_void_check CHECK (
        voided_by_attempt_id IS NULL OR voided_by_attempt_id <> id
    )
);

CREATE INDEX investigation_attempt_user_created_at_index
    ON investigation_attempt (requested_by_user_id, created_at DESC);

CREATE INDEX investigation_attempt_live_positive_index
    ON investigation_attempt (requested_by_user_id, created_at)
    WHERE outcome = 'positive'
      AND consumed_by_activity_id IS NULL
      AND voided_by_attempt_id IS NULL;

CREATE INDEX activity_place_occurred_at_id_index
    ON activity (context_place_entity_id, occurred_at DESC, id DESC)
    WHERE context_place_entity_id IS NOT NULL;

CREATE FUNCTION protect_investigation_attempt() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'investigation attempt history is immutable'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'investigation_attempt_immutable_check';
    END IF;
    IF NEW.id <> OLD.id
       OR NEW.requested_by_user_id <> OLD.requested_by_user_id
       OR NEW.request_id <> OLD.request_id
       OR NEW.character_entity_id <> OLD.character_entity_id
       OR NEW.place_entity_id <> OLD.place_entity_id
       OR NEW.outcome <> OLD.outcome
       OR NEW.created_at <> OLD.created_at
       OR (OLD.consumed_by_activity_id IS NOT NULL
           AND NEW.consumed_by_activity_id IS DISTINCT FROM OLD.consumed_by_activity_id)
       OR (OLD.voided_by_attempt_id IS NOT NULL
           AND NEW.voided_by_attempt_id IS DISTINCT FROM OLD.voided_by_attempt_id)
    THEN
        RAISE EXCEPTION 'investigation attempt identity and accepted lifecycle are immutable'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'investigation_attempt_immutable_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER investigation_attempt_immutable
    BEFORE UPDATE OR DELETE ON investigation_attempt
    FOR EACH ROW EXECUTE FUNCTION protect_investigation_attempt();

CREATE OR REPLACE FUNCTION validate_entity_trait_version_activity() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM inserted_entity_trait_version AS version
        JOIN activity ON activity.id = version.activity_id
        WHERE NOT (
            activity.operation IN (
                'create_character',
                'create_entity',
                'create_entry_place',
                'submit_interaction',
                'submit_discovery'
            )
            OR (
                activity.operation = 'submit_action'
                AND activity.action_consequence IN (
                    'introduce_entity',
                    'change_entity_state'
                )
            )
        )
    ) THEN
        RAISE EXCEPTION 'Trait version Activity is not an Entity creation, state Action, Interaction or discovery'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'entity_trait_version_activity_check';
    END IF;
    RETURN NEW;
END;
$$;
