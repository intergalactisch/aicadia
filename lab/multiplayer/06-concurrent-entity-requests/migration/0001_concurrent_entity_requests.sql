CREATE TABLE place (
    id uuid PRIMARY KEY,
    name text NOT NULL CHECK (octet_length(name) BETWEEN 1 AND 128)
);

CREATE TABLE character (
    id uuid PRIMARY KEY,
    place_id uuid NOT NULL REFERENCES place(id) ON DELETE RESTRICT,
    placement_version bigint NOT NULL CHECK (placement_version > 0)
);

CREATE TABLE entity (
    id uuid PRIMARY KEY,
    place_id uuid NOT NULL REFERENCES place(id) ON DELETE RESTRICT,
    name text NOT NULL CHECK (octet_length(name) BETWEEN 1 AND 128)
);

CREATE TABLE activity (
    id uuid PRIMARY KEY,
    actor_character_id uuid NOT NULL REFERENCES character(id) ON DELETE RESTRICT,
    place_id uuid NOT NULL REFERENCES place(id) ON DELETE RESTRICT,
    operation text NOT NULL CHECK (octet_length(operation) BETWEEN 1 AND 128),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX activity_place_recent_index
    ON activity (place_id, recorded_at DESC, id DESC);

CREATE TABLE property_slot (
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    property_key text NOT NULL CHECK (
        octet_length(property_key) BETWEEN 1 AND 64
        AND property_key ~ '^[a-z][a-z0-9_]*$'
    ),
    current_version bigint,
    current_value jsonb,
    current_activity_id uuid REFERENCES activity(id) ON DELETE RESTRICT,
    PRIMARY KEY (entity_id, property_key),
    CHECK (
        (current_version IS NULL AND current_value IS NULL AND current_activity_id IS NULL)
        OR
        (current_version > 0 AND current_value IS NOT NULL AND current_activity_id IS NOT NULL)
    ),
    CHECK (current_value IS NULL OR pg_column_size(current_value) <= 4096)
);

CREATE TABLE property_history (
    entity_id uuid NOT NULL,
    property_key text NOT NULL,
    version bigint NOT NULL CHECK (version > 0),
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE RESTRICT,
    value jsonb NOT NULL CHECK (pg_column_size(value) <= 4096),
    PRIMARY KEY (entity_id, property_key, version),
    UNIQUE (entity_id, property_key, activity_id),
    FOREIGN KEY (entity_id, property_key)
        REFERENCES property_slot(entity_id, property_key)
        ON DELETE RESTRICT
);

CREATE INDEX property_history_recent_index
    ON property_history (entity_id, property_key, version DESC);

CREATE TABLE activity_dependency (
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE RESTRICT,
    ordinal smallint NOT NULL CHECK (ordinal BETWEEN 0 AND 15),
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    property_key text NOT NULL,
    expected_version bigint,
    expected_value jsonb,
    PRIMARY KEY (activity_id, ordinal),
    UNIQUE (activity_id, entity_id, property_key),
    FOREIGN KEY (entity_id, property_key)
        REFERENCES property_slot(entity_id, property_key)
        ON DELETE RESTRICT,
    CHECK (
        (expected_version IS NULL AND expected_value IS NULL)
        OR
        (expected_version > 0 AND expected_value IS NOT NULL)
    )
);

CREATE TABLE accepted_request (
    actor_character_id uuid NOT NULL REFERENCES character(id) ON DELETE RESTRICT,
    request_id uuid NOT NULL,
    fingerprint bytea NOT NULL CHECK (octet_length(fingerprint) = 32),
    activity_id uuid NOT NULL UNIQUE REFERENCES activity(id) ON DELETE RESTRICT,
    result jsonb NOT NULL CHECK (pg_column_size(result) <= 8192),
    PRIMARY KEY (actor_character_id, request_id)
);
