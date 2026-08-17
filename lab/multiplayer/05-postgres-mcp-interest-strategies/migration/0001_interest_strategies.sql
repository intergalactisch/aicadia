CREATE TABLE resource (
    uri text PRIMARY KEY,
    kind text NOT NULL CHECK (kind IN ('world', 'area', 'place', 'entity')),
    parent_uri text REFERENCES resource(uri) ON DELETE RESTRICT,
    area_id uuid,
    place_id uuid,
    entity_id uuid,
    name text NOT NULL CHECK (octet_length(name) BETWEEN 1 AND 128),
    state jsonb NOT NULL CHECK (octet_length(state::text) <= 4096),
    version uuid NOT NULL,
    updated_activity_id uuid,
    CONSTRAINT resource_shape CHECK (
        (kind = 'world'
            AND parent_uri IS NULL
            AND area_id IS NULL
            AND place_id IS NULL
            AND entity_id IS NULL)
        OR (kind = 'area'
            AND parent_uri IS NOT NULL
            AND area_id IS NOT NULL
            AND place_id IS NULL
            AND entity_id IS NULL)
        OR (kind = 'place'
            AND parent_uri IS NOT NULL
            AND area_id IS NULL
            AND place_id IS NOT NULL
            AND entity_id IS NULL)
        OR (kind = 'entity'
            AND parent_uri IS NOT NULL
            AND area_id IS NULL
            AND place_id IS NULL
            AND entity_id IS NOT NULL)
    )
);

CREATE UNIQUE INDEX resource_one_world_index
    ON resource ((kind))
    WHERE kind = 'world';
CREATE UNIQUE INDEX resource_area_id_index
    ON resource (area_id)
    WHERE kind = 'area';
CREATE UNIQUE INDEX resource_place_id_index
    ON resource (place_id)
    WHERE kind = 'place';
CREATE UNIQUE INDEX resource_entity_id_index
    ON resource (entity_id)
    WHERE kind = 'entity';
CREATE INDEX resource_parent_uri_index ON resource (parent_uri, uri);

CREATE TABLE activity (
    id uuid PRIMARY KEY,
    operation text NOT NULL CHECK (octet_length(operation) BETWEEN 1 AND 128),
    scope_kind text NOT NULL CHECK (scope_kind IN ('local', 'area', 'world')),
    scope_area_id uuid,
    primary_entity_id uuid,
    primary_place_id uuid,
    affected_place_ids uuid[] NOT NULL,
    resource_uris text[] NOT NULL,
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT activity_scope_shape CHECK (
        (scope_kind = 'area' AND scope_area_id IS NOT NULL)
        OR (scope_kind <> 'area' AND scope_area_id IS NULL)
    ),
    CONSTRAINT activity_bounds CHECK (
        cardinality(affected_place_ids) <= 32
        AND cardinality(resource_uris) BETWEEN 1 AND 16
    )
);

CREATE INDEX activity_recorded_at_index
    ON activity (recorded_at DESC, id DESC);

CREATE TABLE activity_resource (
    activity_id uuid NOT NULL REFERENCES activity(id) ON DELETE CASCADE,
    resource_uri text NOT NULL REFERENCES resource(uri) ON DELETE RESTRICT,
    recorded_at timestamptz NOT NULL,
    PRIMARY KEY (activity_id, resource_uri)
);

CREATE INDEX activity_resource_recent_index
    ON activity_resource (resource_uri, recorded_at DESC, activity_id DESC);

ALTER TABLE resource
    ADD CONSTRAINT resource_updated_activity_id_fkey
    FOREIGN KEY (updated_activity_id) REFERENCES activity(id) ON DELETE RESTRICT;
