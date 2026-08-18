ALTER TABLE activity
    ADD COLUMN action_consequence text;

-- Existing Actions could only introduce one Entity. Temporarily suspend the
-- append-only trigger so the exact derivable discriminator can be backfilled;
-- no other Activity fact is rewritten.
ALTER TABLE activity DISABLE TRIGGER activity_immutable;
UPDATE activity
SET action_consequence = 'introduce_entity'
WHERE operation = 'submit_action';
ALTER TABLE activity ENABLE TRIGGER activity_immutable;

ALTER TABLE activity
    ADD CONSTRAINT activity_action_consequence_check CHECK (
        (
            operation = 'submit_action'
            AND action_consequence IS NOT NULL
            AND action_consequence IN ('introduce_entity', 'change_entity_property')
        ) OR (
            operation <> 'submit_action'
            AND action_consequence IS NULL
        )
    );

CREATE TABLE property_key (
    id bigint GENERATED ALWAYS AS IDENTITY,
    key text NOT NULL,
    value_type text NOT NULL,
    first_activity_id uuid NOT NULL,
    CONSTRAINT property_key_pkey PRIMARY KEY (id),
    CONSTRAINT property_key_key_key UNIQUE (key),
    CONSTRAINT property_key_id_value_type_key UNIQUE (id, value_type),
    CONSTRAINT property_key_key_format_check CHECK (
        char_length(key) BETWEEN 1 AND 64
        AND key ~ '^[a-z][a-z0-9]*(_[a-z0-9]+)*$'
    ),
    CONSTRAINT property_key_value_type_check CHECK (value_type IN ('text', 'integer')),
    CONSTRAINT property_key_first_activity_id_fkey
        FOREIGN KEY (first_activity_id) REFERENCES activity(id) ON DELETE RESTRICT
);

CREATE TABLE entity_property_history (
    entity_id uuid NOT NULL,
    property_key_id bigint NOT NULL,
    activity_id uuid NOT NULL,
    previous_activity_id uuid,
    value_type text NOT NULL,
    text_value text,
    integer_value bigint,
    CONSTRAINT entity_property_history_pkey
        PRIMARY KEY (entity_id, property_key_id, activity_id),
    CONSTRAINT entity_property_history_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT entity_property_history_property_key_value_type_fkey
        FOREIGN KEY (property_key_id, value_type)
        REFERENCES property_key(id, value_type) ON DELETE RESTRICT,
    CONSTRAINT entity_property_history_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT entity_property_history_previous_fkey
        FOREIGN KEY (entity_id, property_key_id, previous_activity_id)
        REFERENCES entity_property_history(entity_id, property_key_id, activity_id)
        ON DELETE RESTRICT,
    CONSTRAINT entity_property_history_value_check CHECK (
        (
            value_type = 'text'
            AND text_value IS NOT NULL
            AND text_value = btrim(text_value)
            AND char_length(text_value) BETWEEN 1 AND 4000
            AND integer_value IS NULL
        ) OR (
            value_type = 'integer'
            AND text_value IS NULL
            AND integer_value IS NOT NULL
        )
    )
);

CREATE INDEX entity_property_history_activity_index
    ON entity_property_history (activity_id, entity_id, property_key_id);

CREATE TABLE entity_property (
    entity_id uuid NOT NULL,
    property_key_id bigint NOT NULL,
    current_activity_id uuid NOT NULL,
    CONSTRAINT entity_property_pkey PRIMARY KEY (entity_id, property_key_id),
    CONSTRAINT entity_property_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT entity_property_property_key_id_fkey
        FOREIGN KEY (property_key_id) REFERENCES property_key(id) ON DELETE RESTRICT,
    CONSTRAINT entity_property_current_history_fkey
        FOREIGN KEY (entity_id, property_key_id, current_activity_id)
        REFERENCES entity_property_history(entity_id, property_key_id, activity_id)
        ON DELETE RESTRICT
);

CREATE TRIGGER property_key_immutable
    BEFORE UPDATE OR DELETE ON property_key
    FOR EACH ROW EXECUTE FUNCTION reject_activity_change();

CREATE TRIGGER entity_property_history_immutable
    BEFORE UPDATE OR DELETE ON entity_property_history
    FOR EACH ROW EXECUTE FUNCTION reject_activity_change();
