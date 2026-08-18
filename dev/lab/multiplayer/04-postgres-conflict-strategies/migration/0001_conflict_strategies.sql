CREATE TABLE place (
    id uuid PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE entity (
    id uuid PRIMARY KEY,
    place_id uuid NOT NULL REFERENCES place(id) ON DELETE RESTRICT,
    name text NOT NULL
);

CREATE TABLE activity (
    id uuid PRIMARY KEY,
    strategy text NOT NULL CHECK (
        strategy IN ('place', 'entity', 'hybrid', 'slot', 'serializable')
    ),
    operation text NOT NULL
);

CREATE TABLE property_history (
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    key text NOT NULL,
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE RESTRICT,
    previous_activity_id uuid,
    value text NOT NULL CHECK (value <> '__force_failure__'),
    PRIMARY KEY (entity_id, key, activity_id),
    FOREIGN KEY (entity_id, key, previous_activity_id)
        REFERENCES property_history(entity_id, key, activity_id)
        ON DELETE RESTRICT
);

CREATE UNIQUE INDEX property_history_one_root_index
    ON property_history (entity_id, key)
    WHERE previous_activity_id IS NULL;

CREATE UNIQUE INDEX property_history_one_successor_index
    ON property_history (entity_id, key, previous_activity_id)
    WHERE previous_activity_id IS NOT NULL;

CREATE TABLE property_current (
    entity_id uuid NOT NULL,
    key text NOT NULL,
    current_activity_id uuid NOT NULL,
    PRIMARY KEY (entity_id, key),
    FOREIGN KEY (entity_id, key, current_activity_id)
        REFERENCES property_history(entity_id, key, activity_id)
        ON DELETE RESTRICT
);

CREATE TABLE property_slot (
    entity_id uuid NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
    key text NOT NULL,
    current_activity_id uuid,
    PRIMARY KEY (entity_id, key),
    FOREIGN KEY (entity_id, key, current_activity_id)
        REFERENCES property_history(entity_id, key, activity_id)
        ON DELETE RESTRICT
);

INSERT INTO place (id, name)
VALUES ('00000000-0000-0000-0000-000000000100', 'Old Quarry');

INSERT INTO entity (id, place_id, name)
VALUES
    (
        '00000000-0000-0000-0000-000000000200',
        '00000000-0000-0000-0000-000000000100',
        'Great Stone'
    ),
    (
        '00000000-0000-0000-0000-000000000201',
        '00000000-0000-0000-0000-000000000100',
        'Quarry Door'
    );

INSERT INTO activity (id, strategy, operation)
VALUES
    ('00000000-0000-0000-0000-000000000300', 'hybrid', 'seed'),
    ('00000000-0000-0000-0000-000000000301', 'hybrid', 'seed'),
    ('00000000-0000-0000-0000-000000000302', 'hybrid', 'seed');

INSERT INTO property_history (
    entity_id, key, activity_id, previous_activity_id, value
)
VALUES
    (
        '00000000-0000-0000-0000-000000000200',
        'state',
        '00000000-0000-0000-0000-000000000300',
        NULL,
        'standing'
    ),
    (
        '00000000-0000-0000-0000-000000000200',
        'color',
        '00000000-0000-0000-0000-000000000301',
        NULL,
        'gray'
    ),
    (
        '00000000-0000-0000-0000-000000000201',
        'state',
        '00000000-0000-0000-0000-000000000302',
        NULL,
        'open'
    );

INSERT INTO property_current (entity_id, key, current_activity_id)
VALUES
    (
        '00000000-0000-0000-0000-000000000200',
        'state',
        '00000000-0000-0000-0000-000000000300'
    ),
    (
        '00000000-0000-0000-0000-000000000200',
        'color',
        '00000000-0000-0000-0000-000000000301'
    ),
    (
        '00000000-0000-0000-0000-000000000201',
        'state',
        '00000000-0000-0000-0000-000000000302'
    );

INSERT INTO property_slot (entity_id, key, current_activity_id)
SELECT entity_id, key, current_activity_id
FROM property_current;
