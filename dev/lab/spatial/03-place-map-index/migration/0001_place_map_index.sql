CREATE TABLE place_map_index (
    place_entity_id uuid PRIMARY KEY,
    position_activity_id uuid NOT NULL,
    x_cm bigint NOT NULL,
    y_cm bigint NOT NULL,
    z_cm bigint NOT NULL
);

CREATE INDEX place_map_index_xyz_entity_index
    ON place_map_index (x_cm, y_cm, z_cm, place_entity_id)
    INCLUDE (position_activity_id);
