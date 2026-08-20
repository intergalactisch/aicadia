CREATE TABLE entity (
    id uuid PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE activity (
    id uuid PRIMARY KEY,
    requester_id uuid NOT NULL,
    request_id uuid NOT NULL,
    fingerprint text NOT NULL,
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    operation text NOT NULL CHECK (
        operation IN ('seed_position', 'set_position', 'set_local_state')
    ),
    UNIQUE (requester_id, request_id)
);

CREATE TABLE position_version (
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE RESTRICT,
    previous_activity_id uuid,
    reference_entity_id uuid REFERENCES entity(id) ON DELETE RESTRICT,
    x bigint NOT NULL CHECK (x BETWEEN -9000000000000000 AND 9000000000000000),
    y bigint NOT NULL CHECK (y BETWEEN -9000000000000000 AND 9000000000000000),
    z bigint NOT NULL CHECK (z BETWEEN -9000000000000000 AND 9000000000000000),
    description text,
    PRIMARY KEY (entity_id, activity_id),
    FOREIGN KEY (entity_id, previous_activity_id)
        REFERENCES position_version(entity_id, activity_id)
        ON DELETE RESTRICT,
    CHECK (reference_entity_id IS NULL OR reference_entity_id <> entity_id)
);

CREATE UNIQUE INDEX position_version_one_root_index
    ON position_version (entity_id)
    WHERE previous_activity_id IS NULL;

CREATE UNIQUE INDEX position_version_one_successor_index
    ON position_version (entity_id, previous_activity_id)
    WHERE previous_activity_id IS NOT NULL;

CREATE TABLE position (
    entity_id uuid PRIMARY KEY REFERENCES entity(id) ON DELETE RESTRICT,
    current_activity_id uuid NOT NULL,
    FOREIGN KEY (entity_id, current_activity_id)
        REFERENCES position_version(entity_id, activity_id)
        ON DELETE RESTRICT
);

CREATE TABLE local_state_version (
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE RESTRICT,
    previous_activity_id uuid,
    value text NOT NULL CHECK (value <> '__force_failure__'),
    PRIMARY KEY (entity_id, activity_id),
    FOREIGN KEY (entity_id, previous_activity_id)
        REFERENCES local_state_version(entity_id, activity_id)
        ON DELETE RESTRICT
);

CREATE UNIQUE INDEX local_state_version_one_root_index
    ON local_state_version (entity_id)
    WHERE previous_activity_id IS NULL;

CREATE UNIQUE INDEX local_state_version_one_successor_index
    ON local_state_version (entity_id, previous_activity_id)
    WHERE previous_activity_id IS NOT NULL;

CREATE TABLE local_state (
    entity_id uuid PRIMARY KEY REFERENCES entity(id) ON DELETE RESTRICT,
    current_activity_id uuid NOT NULL,
    FOREIGN KEY (entity_id, current_activity_id)
        REFERENCES local_state_version(entity_id, activity_id)
        ON DELETE RESTRICT
);

CREATE FUNCTION reject_immutable_change()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION '% is immutable', TG_TABLE_NAME;
END;
$$;

CREATE TRIGGER activity_is_immutable
    BEFORE UPDATE OR DELETE ON activity
    FOR EACH ROW EXECUTE FUNCTION reject_immutable_change();

CREATE TRIGGER position_version_is_immutable
    BEFORE UPDATE OR DELETE ON position_version
    FOR EACH ROW EXECUTE FUNCTION reject_immutable_change();

CREATE TRIGGER local_state_version_is_immutable
    BEFORE UPDATE OR DELETE ON local_state_version
    FOR EACH ROW EXECUTE FUNCTION reject_immutable_change();
