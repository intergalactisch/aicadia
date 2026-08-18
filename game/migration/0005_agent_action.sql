CREATE TABLE entity_location (
    entity_id uuid NOT NULL,
    place_entity_id uuid NOT NULL,
    CONSTRAINT entity_location_pkey PRIMARY KEY (entity_id),
    CONSTRAINT entity_location_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT entity_location_place_entity_id_fkey
        FOREIGN KEY (place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT
);

CREATE INDEX entity_location_place_entity_id_entity_id_index
    ON entity_location (place_entity_id, entity_id);

ALTER TABLE activity
    ADD COLUMN prose text,
    ADD COLUMN request_id uuid,
    ADD COLUMN request_fingerprint bytea,
    DROP CONSTRAINT activity_operation_check,
    ADD CONSTRAINT activity_operation_check CHECK (
        operation IN (
            'create_character',
            'create_entity',
            'create_entry_place',
            'enter_world',
            'submit_action'
        )
    ),
    ADD CONSTRAINT activity_action_provenance_check CHECK (
        (
            operation = 'submit_action'
            AND prose IS NOT NULL
            AND request_id IS NOT NULL
            AND request_fingerprint IS NOT NULL
        ) OR (
            operation <> 'submit_action'
            AND prose IS NULL
            AND request_id IS NULL
            AND request_fingerprint IS NULL
        )
    ),
    ADD CONSTRAINT activity_prose_check CHECK (
        prose IS NULL OR (
            prose = btrim(prose)
            AND char_length(prose) BETWEEN 1 AND 4000
        )
    ),
    ADD CONSTRAINT activity_request_fingerprint_check CHECK (
        request_fingerprint IS NULL OR octet_length(request_fingerprint) = 32
    );

CREATE UNIQUE INDEX activity_requested_by_user_id_request_id_index
    ON activity (requested_by_user_id, request_id)
    WHERE request_id IS NOT NULL;

ALTER TABLE activity_entity
    DROP CONSTRAINT activity_entity_role_check,
    ADD CONSTRAINT activity_entity_role_check
        CHECK (role IN ('subject', 'destination', 'location'));

-- Historic Place state has no acceptance sequence beyond its immutable Activities.
-- Seed the pointer to the latest Activity under the historic public ordering, without
-- inventing a new Activity or rewriting a timestamp. Every future relevant writer
-- advances this pointer explicitly while holding the Place lock.
ALTER TABLE place ADD COLUMN latest_activity_id uuid;

UPDATE place
SET latest_activity_id = (
    SELECT activity.id
    FROM activity
    WHERE activity.context_place_entity_id = place.entity_id
       OR EXISTS (
            SELECT 1
            FROM activity_entity
            WHERE activity_entity.activity_id = activity.id
              AND activity_entity.entity_id = place.entity_id
       )
    ORDER BY activity.occurred_at DESC, activity.id DESC
    LIMIT 1
);

ALTER TABLE place
    ALTER COLUMN latest_activity_id SET NOT NULL,
    ADD CONSTRAINT place_latest_activity_id_fkey
        FOREIGN KEY (latest_activity_id) REFERENCES activity(id) ON DELETE RESTRICT;
