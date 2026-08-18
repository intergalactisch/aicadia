ALTER TABLE activity
    DROP CONSTRAINT activity_action_consequence_check,
    ADD CONSTRAINT activity_action_consequence_check CHECK (
        (
            operation = 'submit_action'
            AND action_consequence IS NOT NULL
            AND action_consequence IN (
                'introduce_entity',
                'change_entity_property',
                'change_entity_trait'
            )
        ) OR (
            operation <> 'submit_action'
            AND action_consequence IS NULL
        )
    );

CREATE TABLE entity_trait (
    id uuid NOT NULL,
    entity_id uuid NOT NULL,
    CONSTRAINT entity_trait_pkey PRIMARY KEY (id),
    CONSTRAINT entity_trait_id_entity_id_key UNIQUE (id, entity_id),
    CONSTRAINT entity_trait_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT
);

CREATE INDEX entity_trait_entity_id_id_index
    ON entity_trait (entity_id, id);

CREATE TABLE entity_trait_version (
    trait_id uuid NOT NULL,
    entity_id uuid NOT NULL,
    activity_id uuid NOT NULL,
    previous_activity_id uuid,
    statement text NOT NULL,
    CONSTRAINT entity_trait_version_pkey PRIMARY KEY (trait_id, activity_id),
    CONSTRAINT entity_trait_version_trait_id_entity_id_activity_id_key
        UNIQUE (trait_id, entity_id, activity_id),
    CONSTRAINT entity_trait_version_trait_id_entity_id_fkey
        FOREIGN KEY (trait_id, entity_id)
        REFERENCES entity_trait(id, entity_id) ON DELETE RESTRICT,
    CONSTRAINT entity_trait_version_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT entity_trait_version_previous_fkey
        FOREIGN KEY (trait_id, entity_id, previous_activity_id)
        REFERENCES entity_trait_version(trait_id, entity_id, activity_id)
        ON DELETE RESTRICT,
    CONSTRAINT entity_trait_version_predecessor_check CHECK (
        previous_activity_id IS NULL OR previous_activity_id <> activity_id
    ),
    -- PostgreSQL text rejects U+0000 before a row can reach this CHECK.
    CONSTRAINT entity_trait_version_statement_check CHECK (
        statement = btrim(statement)
        AND char_length(statement) BETWEEN 1 AND 4000
    )
);

CREATE UNIQUE INDEX entity_trait_version_one_root_index
    ON entity_trait_version (trait_id)
    WHERE previous_activity_id IS NULL;

CREATE UNIQUE INDEX entity_trait_version_one_successor_index
    ON entity_trait_version (trait_id, previous_activity_id)
    WHERE previous_activity_id IS NOT NULL;

CREATE INDEX entity_trait_version_activity_entity_trait_index
    ON entity_trait_version (activity_id, entity_id, trait_id);

CREATE TABLE entity_trait_current (
    trait_id uuid NOT NULL,
    entity_id uuid NOT NULL,
    current_activity_id uuid NOT NULL,
    CONSTRAINT entity_trait_current_pkey PRIMARY KEY (trait_id),
    CONSTRAINT entity_trait_current_trait_id_entity_id_fkey
        FOREIGN KEY (trait_id, entity_id)
        REFERENCES entity_trait(id, entity_id) ON DELETE RESTRICT,
    CONSTRAINT entity_trait_current_version_fkey
        FOREIGN KEY (trait_id, entity_id, current_activity_id)
        REFERENCES entity_trait_version(trait_id, entity_id, activity_id)
        ON DELETE RESTRICT
);

CREATE INDEX entity_trait_current_entity_id_trait_id_index
    ON entity_trait_current (entity_id, trait_id);

-- Trait statements are accepted only as explicit Action or Interaction
-- consequences. The Activity exists first, so this trigger keeps that cause closed
-- without copying operation state into the immutable Trait version.
CREATE FUNCTION validate_entity_trait_version_activity() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM inserted_entity_trait_version AS version
        JOIN activity ON activity.id = version.activity_id
        WHERE NOT (
            activity.operation = 'submit_interaction'
            OR (
                activity.operation = 'submit_action'
                AND activity.action_consequence = 'change_entity_trait'
            )
        )
    ) THEN
        RAISE EXCEPTION 'Trait version Activity is not a Trait Action or Interaction'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'entity_trait_version_activity_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER entity_trait_version_activity_check
    AFTER INSERT ON entity_trait_version
    REFERENCING NEW TABLE AS inserted_entity_trait_version
    FOR EACH STATEMENT EXECUTE FUNCTION validate_entity_trait_version_activity();

-- Keep Trait immutability independent from the historical Activity trigger
-- function so migration replay can rebuild older relations in isolation.
CREATE OR REPLACE FUNCTION reject_entity_trait_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'accepted Trait history is immutable'
        USING ERRCODE = '23514';
END;
$$;

CREATE TRIGGER entity_trait_immutable
    BEFORE UPDATE OR DELETE ON entity_trait
    FOR EACH ROW EXECUTE FUNCTION reject_entity_trait_change();

CREATE TRIGGER entity_trait_version_immutable
    BEFORE UPDATE OR DELETE ON entity_trait_version
    FOR EACH ROW EXECUTE FUNCTION reject_entity_trait_change();

CREATE FUNCTION reject_entity_trait_current_identity_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.trait_id <> OLD.trait_id OR NEW.entity_id <> OLD.entity_id THEN
        RAISE EXCEPTION 'current Trait identity and Entity are immutable'
            USING ERRCODE = '23514';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER entity_trait_current_identity_immutable
    BEFORE UPDATE ON entity_trait_current
    FOR EACH ROW EXECUTE FUNCTION reject_entity_trait_current_identity_change();

-- Retirement is absent: by transaction end every stable Trait must have exactly
-- one root, one current pointer and no successor after that current version. Keep
-- this deferred so the set writer can insert identity, version and pointer rows in
-- their foreign-key order inside one transaction. Each invocation validates only
-- the affected Trait id; the primary/partial indexes make every lookup bounded to
-- that lineage.
CREATE FUNCTION validate_entity_trait_complete() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    affected_trait_id uuid;
BEGIN
    IF TG_TABLE_NAME = 'entity_trait' THEN
        affected_trait_id := CASE WHEN TG_OP = 'DELETE' THEN OLD.id ELSE NEW.id END;
    ELSE
        affected_trait_id := CASE WHEN TG_OP = 'DELETE' THEN OLD.trait_id ELSE NEW.trait_id END;
    END IF;
    IF EXISTS (
        SELECT 1
        FROM entity_trait AS trait
        WHERE trait.id = affected_trait_id
          AND (
                (
                    SELECT count(*)
                    FROM entity_trait_version AS version
                    WHERE version.trait_id = trait.id
                      AND version.previous_activity_id IS NULL
                ) <> 1
                OR (
                    SELECT count(*)
                    FROM entity_trait_current AS current
                    WHERE current.trait_id = trait.id
                ) <> 1
                OR EXISTS (
                    SELECT 1
                    FROM entity_trait_current AS current
                    JOIN entity_trait_version AS successor
                      ON successor.trait_id = current.trait_id
                     AND successor.previous_activity_id = current.current_activity_id
                    WHERE current.trait_id = trait.id
                )
              )
    ) THEN
        RAISE EXCEPTION 'Trait lineage must have one root and one current tip'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'entity_trait_complete_check';
    END IF;
    RETURN NULL;
END;
$$;

CREATE CONSTRAINT TRIGGER entity_trait_complete_check
    AFTER INSERT ON entity_trait
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_entity_trait_complete();

CREATE CONSTRAINT TRIGGER entity_trait_version_complete_check
    AFTER INSERT ON entity_trait_version
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_entity_trait_complete();

CREATE CONSTRAINT TRIGGER entity_trait_current_complete_check
    AFTER INSERT OR UPDATE OR DELETE ON entity_trait_current
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_entity_trait_complete();
