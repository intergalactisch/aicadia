ALTER TABLE activity
    DROP CONSTRAINT activity_action_consequence_check,
    ADD CONSTRAINT activity_action_consequence_check CHECK (
        (
            operation = 'submit_action'
            AND action_consequence IS NOT NULL
            AND action_consequence IN (
                'introduce_entity',
                'change_entity_property',
                'change_entity_trait',
                'change_entity_state'
            )
        ) OR (
            operation <> 'submit_action'
            AND action_consequence IS NULL
        )
    );

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
                'submit_interaction'
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
        RAISE EXCEPTION 'Trait version Activity is not an Entity creation, state Action or Interaction'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'entity_trait_version_activity_check';
    END IF;
    RETURN NEW;
END;
$$;
