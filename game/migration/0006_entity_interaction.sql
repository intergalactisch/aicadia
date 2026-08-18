ALTER TABLE activity
    DROP CONSTRAINT activity_operation_check,
    DROP CONSTRAINT activity_action_provenance_check,
    ADD CONSTRAINT activity_operation_check CHECK (
        operation IN (
            'create_character',
            'create_entity',
            'create_entry_place',
            'enter_world',
            'submit_action',
            'submit_interaction'
        )
    ),
    ADD CONSTRAINT activity_confirmed_mutation_provenance_check CHECK (
        (
            operation IN ('submit_action', 'submit_interaction')
            AND prose IS NOT NULL
            AND request_id IS NOT NULL
            AND request_fingerprint IS NOT NULL
        ) OR (
            operation NOT IN ('submit_action', 'submit_interaction')
            AND prose IS NULL
            AND request_id IS NULL
            AND request_fingerprint IS NULL
        )
    ),
    ADD CONSTRAINT activity_interaction_context_check CHECK (
        operation <> 'submit_interaction'
        OR (
            actor_character_entity_id IS NOT NULL
            AND context_place_entity_id IS NOT NULL
        )
    );

ALTER TABLE activity_entity
    DROP CONSTRAINT activity_entity_role_check,
    ADD CONSTRAINT activity_entity_role_check
        CHECK (role IN ('subject', 'destination', 'location', 'target'));

CREATE INDEX character_current_place_entity_id_entity_id_index
    ON character (current_place_entity_id, entity_id)
    WHERE current_place_entity_id IS NOT NULL;
