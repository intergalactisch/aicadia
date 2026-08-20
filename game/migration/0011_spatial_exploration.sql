-- Refuse to invent legacy spatial truth. Every old positioned subject must have
-- exactly one already-stored Activity that establishes its current relation.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM place
        WHERE NOT is_entry
    ) OR EXISTS (
        SELECT 1
        FROM place
        LEFT JOIN LATERAL (
            SELECT count(*) AS candidate_count
            FROM activity
            JOIN activity_entity
              ON activity_entity.activity_id = activity.id
             AND activity_entity.entity_id = place.entity_id
             AND activity_entity.role = 'subject'
            WHERE activity.operation = 'create_entry_place'
        ) candidate ON true
        WHERE candidate.candidate_count <> 1
    ) THEN
        RAISE EXCEPTION 'legacy Place Position cannot be established exactly'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_backfill_place_check';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM character
        LEFT JOIN LATERAL (
            SELECT count(*) AS candidate_count
            FROM activity
            JOIN activity_entity
              ON activity_entity.activity_id = activity.id
             AND activity_entity.entity_id = character.current_place_entity_id
             AND activity_entity.role = 'destination'
            WHERE activity.operation = 'enter_world'
              AND activity.actor_character_entity_id = character.entity_id
              AND activity.context_place_entity_id = character.current_place_entity_id
        ) candidate ON true
        WHERE character.current_place_entity_id IS NOT NULL
          AND candidate.candidate_count <> 1
    ) THEN
        RAISE EXCEPTION 'legacy Character Position cannot be established exactly'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_backfill_character_check';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM entity_location
        LEFT JOIN LATERAL (
            SELECT count(*) AS candidate_count
            FROM activity
            JOIN activity_entity subject
              ON subject.activity_id = activity.id
             AND subject.entity_id = entity_location.entity_id
             AND subject.role = 'subject'
            JOIN activity_entity location
              ON location.activity_id = activity.id
             AND location.entity_id = entity_location.place_entity_id
             AND location.role = 'location'
            WHERE activity.context_place_entity_id = entity_location.place_entity_id
              AND (
                    (activity.operation = 'submit_action'
                     AND activity.action_consequence = 'introduce_entity')
                    OR activity.operation = 'submit_discovery'
                  )
        ) candidate ON true
        WHERE candidate.candidate_count <> 1
    ) THEN
        RAISE EXCEPTION 'legacy Entity Position cannot be established exactly'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_backfill_entity_check';
    END IF;
END;
$$;

ALTER TABLE activity
    DROP CONSTRAINT activity_operation_check,
    DROP CONSTRAINT activity_confirmed_mutation_provenance_check,
    ADD CONSTRAINT activity_operation_check CHECK (
        operation IN (
            'create_character',
            'create_entity',
            'create_entry_place',
            'enter_world',
            'submit_action',
            'submit_interaction',
            'submit_discovery',
            'move_character'
        )
    ),
    ADD CONSTRAINT activity_confirmed_mutation_provenance_check CHECK (
        (
            operation IN ('submit_action', 'submit_interaction', 'submit_discovery')
            AND prose IS NOT NULL
            AND request_id IS NOT NULL
            AND request_fingerprint IS NOT NULL
        ) OR (
            operation = 'move_character'
            AND prose IS NULL
            AND request_id IS NOT NULL
            AND request_fingerprint IS NOT NULL
        ) OR (
            operation NOT IN (
                'submit_action', 'submit_interaction', 'submit_discovery', 'move_character'
            )
            AND prose IS NULL
            AND request_id IS NULL
            AND request_fingerprint IS NULL
        )
    );

CREATE TABLE position_version (
    entity_id uuid NOT NULL,
    activity_id uuid NOT NULL,
    previous_activity_id uuid,
    x_cm bigint NOT NULL,
    y_cm bigint NOT NULL,
    z_cm bigint NOT NULL,
    description text,
    CONSTRAINT position_version_pkey PRIMARY KEY (entity_id, activity_id),
    CONSTRAINT position_version_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT position_version_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT position_version_previous_fkey
        FOREIGN KEY (entity_id, previous_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT,
    CONSTRAINT position_version_predecessor_check CHECK (
        previous_activity_id IS NULL OR previous_activity_id <> activity_id
    ),
    CONSTRAINT position_version_coordinate_check CHECK (
        x_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND y_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND z_cm BETWEEN -1000000000000000 AND 1000000000000000
    ),
    CONSTRAINT position_version_description_check CHECK (
        description IS NULL OR (
            description = btrim(description)
            AND char_length(description) BETWEEN 1 AND 4000
        )
    )
);

CREATE UNIQUE INDEX position_version_one_root_index
    ON position_version (entity_id)
    WHERE previous_activity_id IS NULL;

CREATE UNIQUE INDEX position_version_one_successor_index
    ON position_version (entity_id, previous_activity_id)
    WHERE previous_activity_id IS NOT NULL;

CREATE TABLE position (
    entity_id uuid NOT NULL,
    current_activity_id uuid NOT NULL,
    CONSTRAINT position_pkey PRIMARY KEY (entity_id),
    CONSTRAINT position_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT position_current_version_fkey
        FOREIGN KEY (entity_id, current_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT
);

CREATE TABLE activity_position (
    activity_id uuid NOT NULL,
    role text NOT NULL,
    position_entity_id uuid NOT NULL,
    position_activity_id uuid NOT NULL,
    CONSTRAINT activity_position_pkey PRIMARY KEY (
        activity_id, role, position_entity_id, position_activity_id
    ),
    CONSTRAINT activity_position_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT activity_position_version_fkey
        FOREIGN KEY (position_entity_id, position_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT,
    CONSTRAINT activity_position_role_check CHECK (role IN ('origin', 'result')),
    CONSTRAINT activity_position_result_activity_check CHECK (
        role <> 'result' OR activity_id = position_activity_id
    )
);

CREATE INDEX activity_position_position_activity_index
    ON activity_position (position_entity_id, position_activity_id, activity_id);

-- Every old Place was the sole genesis Place, so its exact established point is
-- World origin. Keep these candidates temporary: canonical truth starts in the
-- version relations, never in a migration helper.
CREATE TEMPORARY TABLE spatial_position_backfill (
    entity_id uuid PRIMARY KEY,
    activity_id uuid NOT NULL,
    x_cm bigint NOT NULL,
    y_cm bigint NOT NULL,
    z_cm bigint NOT NULL
) ON COMMIT DROP;

INSERT INTO spatial_position_backfill (entity_id, activity_id, x_cm, y_cm, z_cm)
SELECT place.entity_id, activity.id, 0, 0, 0
FROM place
JOIN activity_entity
  ON activity_entity.entity_id = place.entity_id
 AND activity_entity.role = 'subject'
JOIN activity
  ON activity.id = activity_entity.activity_id
 AND activity.operation = 'create_entry_place';

INSERT INTO spatial_position_backfill (entity_id, activity_id, x_cm, y_cm, z_cm)
SELECT character.entity_id, activity.id,
       place_position.x_cm, place_position.y_cm, place_position.z_cm
FROM character
JOIN activity
  ON activity.operation = 'enter_world'
 AND activity.actor_character_entity_id = character.entity_id
 AND activity.context_place_entity_id = character.current_place_entity_id
JOIN activity_entity destination
  ON destination.activity_id = activity.id
 AND destination.entity_id = character.current_place_entity_id
 AND destination.role = 'destination'
JOIN spatial_position_backfill place_position
  ON place_position.entity_id = character.current_place_entity_id
WHERE character.current_place_entity_id IS NOT NULL;

INSERT INTO spatial_position_backfill (entity_id, activity_id, x_cm, y_cm, z_cm)
SELECT entity_location.entity_id, activity.id,
       place_position.x_cm, place_position.y_cm, place_position.z_cm
FROM entity_location
JOIN activity_entity subject
  ON subject.entity_id = entity_location.entity_id
 AND subject.role = 'subject'
JOIN activity ON activity.id = subject.activity_id
JOIN activity_entity location
  ON location.activity_id = activity.id
 AND location.entity_id = entity_location.place_entity_id
 AND location.role = 'location'
JOIN spatial_position_backfill place_position
  ON place_position.entity_id = entity_location.place_entity_id
WHERE activity.context_place_entity_id = entity_location.place_entity_id
  AND (
        (activity.operation = 'submit_action'
         AND activity.action_consequence = 'introduce_entity')
        OR activity.operation = 'submit_discovery'
      );

INSERT INTO position_version (
    entity_id, activity_id, previous_activity_id, x_cm, y_cm, z_cm, description
)
SELECT entity_id, activity_id, NULL, x_cm, y_cm, z_cm, NULL
FROM spatial_position_backfill;

INSERT INTO position (entity_id, current_activity_id)
SELECT entity_id, activity_id
FROM spatial_position_backfill;

INSERT INTO activity_position (
    activity_id, role, position_entity_id, position_activity_id
)
SELECT activity_id, 'result', entity_id, activity_id
FROM spatial_position_backfill;

ALTER TABLE place
    ADD CONSTRAINT place_position_fkey
        FOREIGN KEY (entity_id) REFERENCES position(entity_id) ON DELETE RESTRICT;

ALTER TABLE entity_location
    ADD CONSTRAINT entity_location_position_fkey
        FOREIGN KEY (entity_id) REFERENCES position(entity_id) ON DELETE RESTRICT;

CREATE OR REPLACE FUNCTION reject_position_version_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'accepted Position history is immutable'
        USING ERRCODE = '23514',
              CONSTRAINT = 'position_version_immutable_check';
END;
$$;

CREATE TRIGGER position_version_immutable
    BEFORE UPDATE OR DELETE ON position_version
    FOR EACH ROW EXECUTE FUNCTION reject_position_version_change();

CREATE OR REPLACE FUNCTION protect_position_current() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'Position removal is absent'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_current_immutable_check';
    END IF;
    IF NEW.entity_id <> OLD.entity_id OR EXISTS (
        SELECT 1 FROM place WHERE place.entity_id = OLD.entity_id
    ) OR NOT EXISTS (
        SELECT 1
        FROM position_version successor
        WHERE successor.entity_id = OLD.entity_id
          AND successor.activity_id = NEW.current_activity_id
          AND successor.previous_activity_id = OLD.current_activity_id
    ) THEN
        RAISE EXCEPTION 'current Position may only advance to one direct successor'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_current_advance_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER position_current_protected
    BEFORE UPDATE OR DELETE ON position
    FOR EACH ROW EXECUTE FUNCTION protect_position_current();

CREATE OR REPLACE FUNCTION reject_spatial_history_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'accepted spatial history is immutable'
        USING ERRCODE = '23514',
              CONSTRAINT = 'spatial_history_immutable_check';
END;
$$;

CREATE TRIGGER activity_position_immutable
    BEFORE UPDATE OR DELETE ON activity_position
    FOR EACH ROW EXECUTE FUNCTION reject_spatial_history_change();

CREATE OR REPLACE FUNCTION validate_position_predecessor_current() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.previous_activity_id = NEW.activity_id THEN
        RAISE EXCEPTION 'Position version cannot name itself as predecessor'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_cycle_check';
    END IF;
    IF NEW.previous_activity_id IS NOT NULL AND NOT EXISTS (
        SELECT 1
        FROM position
        WHERE position.entity_id = NEW.entity_id
          AND position.current_activity_id = NEW.previous_activity_id
    ) THEN
        RAISE EXCEPTION 'new Position version must extend the exact current pointer'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_predecessor_current_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER position_version_predecessor_current_check
    BEFORE INSERT ON position_version
    FOR EACH ROW EXECUTE FUNCTION validate_position_predecessor_current();

CREATE OR REPLACE FUNCTION validate_new_position_complete() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM position
        WHERE position.entity_id = NEW.entity_id
          AND position.current_activity_id = NEW.activity_id
    ) OR NOT EXISTS (
        SELECT 1
        FROM activity_position
        WHERE activity_position.activity_id = NEW.activity_id
          AND activity_position.role = 'result'
          AND activity_position.position_entity_id = NEW.entity_id
          AND activity_position.position_activity_id = NEW.activity_id
    ) THEN
        RAISE EXCEPTION 'new Position version requires its typed result and current pointer'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'position_complete_check';
    END IF;
    RETURN NULL;
END;
$$;

CREATE CONSTRAINT TRIGGER position_version_complete_check
    AFTER INSERT ON position_version
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_new_position_complete();

CREATE TABLE connection (
    id uuid NOT NULL,
    source_place_entity_id uuid NOT NULL,
    destination_place_entity_id uuid NOT NULL,
    source_position_activity_id uuid NOT NULL,
    destination_position_activity_id uuid NOT NULL,
    allows_reverse boolean NOT NULL,
    has_course boolean NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    shape_description text,
    created_by_activity_id uuid NOT NULL,
    CONSTRAINT connection_pkey PRIMARY KEY (id),
    CONSTRAINT connection_source_place_entity_id_fkey
        FOREIGN KEY (source_place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT,
    CONSTRAINT connection_destination_place_entity_id_fkey
        FOREIGN KEY (destination_place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT,
    CONSTRAINT connection_source_position_fkey
        FOREIGN KEY (source_place_entity_id, source_position_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT,
    CONSTRAINT connection_destination_position_fkey
        FOREIGN KEY (destination_place_entity_id, destination_position_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT,
    CONSTRAINT connection_created_by_activity_id_key UNIQUE (created_by_activity_id),
    CONSTRAINT connection_created_by_activity_id_fkey
        FOREIGN KEY (created_by_activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT connection_distinct_endpoint_check CHECK (
        source_place_entity_id <> destination_place_entity_id
    ),
    CONSTRAINT connection_name_check CHECK (
        name = btrim(name) AND char_length(name) BETWEEN 1 AND 120
    ),
    CONSTRAINT connection_description_check CHECK (
        description = btrim(description) AND char_length(description) BETWEEN 1 AND 4000
    ),
    CONSTRAINT connection_shape_description_check CHECK (
        shape_description IS NULL OR (
            shape_description = btrim(shape_description)
            AND char_length(shape_description) BETWEEN 1 AND 4000
        )
    )
);

CREATE INDEX connection_source_place_id_index
    ON connection (source_place_entity_id, id);

CREATE INDEX connection_destination_place_id_index
    ON connection (destination_place_entity_id, id);

CREATE TABLE connection_point (
    connection_id uuid NOT NULL,
    ordinal smallint NOT NULL,
    x_cm bigint NOT NULL,
    y_cm bigint NOT NULL,
    z_cm bigint NOT NULL,
    CONSTRAINT connection_point_pkey PRIMARY KEY (connection_id, ordinal),
    CONSTRAINT connection_point_connection_id_fkey
        FOREIGN KEY (connection_id) REFERENCES connection(id) ON DELETE RESTRICT,
    CONSTRAINT connection_point_ordinal_check CHECK (ordinal BETWEEN 0 AND 127),
    CONSTRAINT connection_point_coordinate_check CHECK (
        x_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND y_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND z_cm BETWEEN -1000000000000000 AND 1000000000000000
    )
);

CREATE TABLE activity_connection (
    activity_id uuid NOT NULL,
    connection_id uuid NOT NULL,
    CONSTRAINT activity_connection_pkey PRIMARY KEY (activity_id),
    CONSTRAINT activity_connection_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT activity_connection_connection_id_fkey
        FOREIGN KEY (connection_id) REFERENCES connection(id) ON DELETE RESTRICT
);

CREATE INDEX activity_connection_connection_activity_index
    ON activity_connection (connection_id, activity_id);

CREATE OR REPLACE FUNCTION reject_connection_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'accepted Connection state is immutable'
        USING ERRCODE = '23514',
              CONSTRAINT = 'connection_immutable_check';
END;
$$;

CREATE TRIGGER connection_immutable
    BEFORE UPDATE OR DELETE ON connection
    FOR EACH ROW EXECUTE FUNCTION reject_connection_change();

CREATE TRIGGER connection_point_immutable
    BEFORE UPDATE OR DELETE ON connection_point
    FOR EACH ROW EXECUTE FUNCTION reject_connection_change();

CREATE TRIGGER activity_connection_immutable
    BEFORE UPDATE OR DELETE ON activity_connection
    FOR EACH ROW EXECUTE FUNCTION reject_spatial_history_change();

CREATE OR REPLACE FUNCTION validate_connection_creator_operation() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM activity
        WHERE activity.id = NEW.created_by_activity_id
          AND activity.operation = 'submit_discovery'
    ) THEN
        RAISE EXCEPTION 'Connection creator must be one submit_discovery Activity'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'connection_creator_operation_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER connection_creator_operation_check
    BEFORE INSERT ON connection
    FOR EACH ROW EXECUTE FUNCTION validate_connection_creator_operation();

CREATE OR REPLACE FUNCTION validate_activity_connection_type() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    activity_operation text;
BEGIN
    SELECT operation INTO activity_operation
    FROM activity
    WHERE id = NEW.activity_id;
    IF activity_operation NOT IN ('submit_discovery', 'move_character') THEN
        RAISE EXCEPTION 'Activity operation cannot reference a Connection'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'activity_connection_operation_check';
    END IF;
    IF activity_operation = 'submit_discovery' AND NOT EXISTS (
        SELECT 1
        FROM connection
        WHERE connection.id = NEW.connection_id
          AND connection.created_by_activity_id = NEW.activity_id
    ) THEN
        RAISE EXCEPTION 'Discovery Activity must reference exactly its created Connection'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'activity_connection_discovery_owner_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER activity_connection_type_check
    BEFORE INSERT ON activity_connection
    FOR EACH ROW EXECUTE FUNCTION validate_activity_connection_type();

CREATE OR REPLACE FUNCTION validate_move_connection_complete() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.operation = 'move_character' AND NOT EXISTS (
        SELECT 1
        FROM activity_connection
        WHERE activity_connection.activity_id = NEW.id
    ) THEN
        RAISE EXCEPTION 'Movement Activity requires exactly one traversed Connection'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'activity_connection_move_required_check';
    END IF;
    RETURN NULL;
END;
$$;

CREATE CONSTRAINT TRIGGER activity_connection_move_required_check
    AFTER INSERT ON activity
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW
    WHEN (NEW.operation = 'move_character')
    EXECUTE FUNCTION validate_move_connection_complete();

CREATE OR REPLACE FUNCTION validate_connection_endpoint_current() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM position source_position
        JOIN position destination_position ON true
        WHERE source_position.entity_id = NEW.source_place_entity_id
          AND source_position.current_activity_id = NEW.source_position_activity_id
          AND destination_position.entity_id = NEW.destination_place_entity_id
          AND destination_position.current_activity_id = NEW.destination_position_activity_id
    ) THEN
        RAISE EXCEPTION 'Connection endpoints must name current Place Positions'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'connection_endpoint_current_check';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER connection_endpoint_current_check
    BEFORE INSERT ON connection
    FOR EACH ROW EXECUTE FUNCTION validate_connection_endpoint_current();

-- Exact 3D segment intersection over PostgreSQL numeric avoids bigint overflow.
CREATE OR REPLACE FUNCTION spatial_segments_intersect(
    ax bigint, ay bigint, az bigint, bx bigint, by bigint, bz bigint,
    cx bigint, cy bigint, cz bigint, dx bigint, dy bigint, dz bigint
) RETURNS boolean
LANGUAGE plpgsql IMMUTABLE STRICT AS $$
DECLARE
    ux numeric := bx::numeric - ax; uy numeric := by::numeric - ay; uz numeric := bz::numeric - az;
    vx numeric := dx::numeric - cx; vy numeric := dy::numeric - cy; vz numeric := dz::numeric - cz;
    wx numeric := cx::numeric - ax; wy numeric := cy::numeric - ay; wz numeric := cz::numeric - az;
    nx numeric; ny numeric; nz numeric; n2 numeric;
    qx numeric; qy numeric; qz numeric;
    t_num numeric; s_num numeric;
BEGIN
    nx := uy * vz - uz * vy;
    ny := uz * vx - ux * vz;
    nz := ux * vy - uy * vx;
    n2 := nx * nx + ny * ny + nz * nz;
    IF n2 = 0 THEN
        qx := wy * uz - wz * uy;
        qy := wz * ux - wx * uz;
        qz := wx * uy - wy * ux;
        IF qx <> 0 OR qy <> 0 OR qz <> 0 THEN
            RETURN false;
        END IF;
        IF ux <> 0 THEN
            RETURN greatest(least(ax, bx), least(cx, dx)) <= least(greatest(ax, bx), greatest(cx, dx));
        ELSIF uy <> 0 THEN
            RETURN greatest(least(ay, by), least(cy, dy)) <= least(greatest(ay, by), greatest(cy, dy));
        ELSE
            RETURN greatest(least(az, bz), least(cz, dz)) <= least(greatest(az, bz), greatest(cz, dz));
        END IF;
    END IF;
    IF wx * nx + wy * ny + wz * nz <> 0 THEN
        RETURN false;
    END IF;
    t_num := (wy * vz - wz * vy) * nx
           + (wz * vx - wx * vz) * ny
           + (wx * vy - wy * vx) * nz;
    s_num := (wy * uz - wz * uy) * nx
           + (wz * ux - wx * uz) * ny
           + (wx * uy - wy * ux) * nz;
    RETURN t_num BETWEEN 0 AND n2 AND s_num BETWEEN 0 AND n2;
END;
$$;

CREATE OR REPLACE FUNCTION validate_connection_complete() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
    affected_connection_id uuid;
    point_count integer;
BEGIN
    IF TG_TABLE_NAME = 'connection' THEN
        IF TG_OP = 'DELETE' THEN
            affected_connection_id := OLD.id;
        ELSE
            affected_connection_id := NEW.id;
        END IF;
    ELSE
        IF TG_OP = 'DELETE' THEN
            affected_connection_id := OLD.connection_id;
        ELSE
            affected_connection_id := NEW.connection_id;
        END IF;
    END IF;
    SELECT count(*) INTO point_count
    FROM connection_point
    WHERE connection_id = affected_connection_id;
    IF EXISTS (SELECT 1 FROM connection WHERE id = affected_connection_id) AND (
        EXISTS (
            SELECT 1
            FROM connection
            WHERE connection.id = affected_connection_id
              AND (
                    (NOT connection.has_course AND point_count <> 0)
                    OR (connection.has_course AND point_count NOT BETWEEN 2 AND 128)
                  )
        )
        OR point_count > 0 AND EXISTS (
            SELECT 1
            FROM connection
            JOIN position_version source_position
              ON source_position.entity_id = connection.source_place_entity_id
             AND source_position.activity_id = connection.source_position_activity_id
            JOIN position_version destination_position
              ON destination_position.entity_id = connection.destination_place_entity_id
             AND destination_position.activity_id = connection.destination_position_activity_id
            WHERE connection.id = affected_connection_id
              AND (
                    (SELECT min(ordinal) FROM connection_point WHERE connection_id = connection.id) <> 0
                    OR (SELECT max(ordinal) FROM connection_point WHERE connection_id = connection.id) <> point_count - 1
                    OR NOT EXISTS (
                        SELECT 1 FROM connection_point first_point
                        WHERE first_point.connection_id = connection.id
                          AND first_point.ordinal = 0
                          AND (first_point.x_cm, first_point.y_cm, first_point.z_cm) =
                              (source_position.x_cm, source_position.y_cm, source_position.z_cm)
                    )
                    OR NOT EXISTS (
                        SELECT 1 FROM connection_point last_point
                        WHERE last_point.connection_id = connection.id
                          AND last_point.ordinal = point_count - 1
                          AND (last_point.x_cm, last_point.y_cm, last_point.z_cm) =
                              (destination_position.x_cm, destination_position.y_cm, destination_position.z_cm)
                    )
                  )
        )
        OR EXISTS (
            SELECT 1
            FROM connection_point first_point
            JOIN connection_point second_point
              ON second_point.connection_id = first_point.connection_id
             AND second_point.ordinal = first_point.ordinal + 1
            WHERE first_point.connection_id = affected_connection_id
              AND (first_point.x_cm, first_point.y_cm, first_point.z_cm) =
                  (second_point.x_cm, second_point.y_cm, second_point.z_cm)
        )
        OR EXISTS (
            SELECT 1
            FROM connection_point first_point
            JOIN connection_point shared_point
              ON shared_point.connection_id = first_point.connection_id
             AND shared_point.ordinal = first_point.ordinal + 1
            JOIN connection_point last_point
              ON last_point.connection_id = shared_point.connection_id
             AND last_point.ordinal = shared_point.ordinal + 1
            WHERE first_point.connection_id = affected_connection_id
              AND (
                    (shared_point.y_cm::numeric - first_point.y_cm) *
                    (last_point.z_cm::numeric - shared_point.z_cm) -
                    (shared_point.z_cm::numeric - first_point.z_cm) *
                    (last_point.y_cm::numeric - shared_point.y_cm) = 0
                AND (shared_point.z_cm::numeric - first_point.z_cm) *
                    (last_point.x_cm::numeric - shared_point.x_cm) -
                    (shared_point.x_cm::numeric - first_point.x_cm) *
                    (last_point.z_cm::numeric - shared_point.z_cm) = 0
                AND (shared_point.x_cm::numeric - first_point.x_cm) *
                    (last_point.y_cm::numeric - shared_point.y_cm) -
                    (shared_point.y_cm::numeric - first_point.y_cm) *
                    (last_point.x_cm::numeric - shared_point.x_cm) = 0
                AND (shared_point.x_cm::numeric - first_point.x_cm) *
                    (last_point.x_cm::numeric - shared_point.x_cm) +
                    (shared_point.y_cm::numeric - first_point.y_cm) *
                    (last_point.y_cm::numeric - shared_point.y_cm) +
                    (shared_point.z_cm::numeric - first_point.z_cm) *
                    (last_point.z_cm::numeric - shared_point.z_cm) < 0
                  )
        )
        OR EXISTS (
            SELECT 1
            FROM connection_point a
            JOIN connection_point b
              ON b.connection_id = a.connection_id AND b.ordinal = a.ordinal + 1
            JOIN connection_point c
              ON c.connection_id = a.connection_id AND c.ordinal > a.ordinal + 1
            JOIN connection_point d
              ON d.connection_id = c.connection_id AND d.ordinal = c.ordinal + 1
            WHERE a.connection_id = affected_connection_id
              AND spatial_segments_intersect(
                    a.x_cm, a.y_cm, a.z_cm, b.x_cm, b.y_cm, b.z_cm,
                    c.x_cm, c.y_cm, c.z_cm, d.x_cm, d.y_cm, d.z_cm
                  )
        )
        OR NOT EXISTS (
            SELECT 1
            FROM connection
            JOIN activity_connection
              ON activity_connection.activity_id = connection.created_by_activity_id
             AND activity_connection.connection_id = connection.id
            WHERE connection.id = affected_connection_id
        )
    ) THEN
        RAISE EXCEPTION 'Connection course and typed Activity must be complete'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'connection_complete_check';
    END IF;
    RETURN NULL;
END;
$$;

CREATE CONSTRAINT TRIGGER connection_complete_check
    AFTER INSERT ON connection
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_connection_complete();

CREATE CONSTRAINT TRIGGER connection_point_complete_check
    AFTER INSERT OR UPDATE OR DELETE ON connection_point
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_connection_complete();

CREATE CONSTRAINT TRIGGER activity_connection_complete_check
    AFTER INSERT OR UPDATE OR DELETE ON activity_connection
    DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION validate_connection_complete();

CREATE TABLE place_map_index (
    place_entity_id uuid NOT NULL,
    position_activity_id uuid NOT NULL,
    x_cm bigint NOT NULL,
    y_cm bigint NOT NULL,
    z_cm bigint NOT NULL,
    CONSTRAINT place_map_index_pkey PRIMARY KEY (place_entity_id),
    CONSTRAINT place_map_index_place_entity_id_fkey
        FOREIGN KEY (place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT,
    CONSTRAINT place_map_index_position_fkey
        FOREIGN KEY (place_entity_id, position_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT,
    CONSTRAINT place_map_index_coordinate_check CHECK (
        x_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND y_cm BETWEEN -1000000000000000 AND 1000000000000000
        AND z_cm BETWEEN -1000000000000000 AND 1000000000000000
    )
);

CREATE INDEX place_map_index_x_y_z_place_covering_index
    ON place_map_index (x_cm, y_cm, z_cm, place_entity_id)
    INCLUDE (position_activity_id);

CREATE INDEX place_map_index_y_z_x_place_covering_index
    ON place_map_index (y_cm, z_cm, x_cm, place_entity_id)
    INCLUDE (position_activity_id);

CREATE INDEX place_map_index_z_x_y_place_covering_index
    ON place_map_index (z_cm, x_cm, y_cm, place_entity_id)
    INCLUDE (position_activity_id);

INSERT INTO place_map_index (
    place_entity_id, position_activity_id, x_cm, y_cm, z_cm
)
SELECT place.entity_id, position.current_activity_id,
       version.x_cm, version.y_cm, version.z_cm
FROM place
JOIN position ON position.entity_id = place.entity_id
JOIN position_version version
  ON version.entity_id = position.entity_id
 AND version.activity_id = position.current_activity_id;

-- Existing Investigation is the entity-at-position kind. Before Movement existed,
-- its stored Place and the Character's current Position are the exact same start
-- grounding, so the migration can bind old attempts without inference.
ALTER TABLE investigation_attempt
    ADD COLUMN kind text NOT NULL DEFAULT 'entity_at_position',
    ADD COLUMN position_activity_id uuid;

UPDATE investigation_attempt attempt
SET position_activity_id = position.current_activity_id
FROM position
WHERE position.entity_id = attempt.character_entity_id;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM investigation_attempt WHERE position_activity_id IS NULL
    ) THEN
        RAISE EXCEPTION 'legacy Investigation Position cannot be established exactly'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'investigation_attempt_position_backfill_check';
    END IF;
END;
$$;

ALTER TABLE investigation_attempt
    ALTER COLUMN position_activity_id SET NOT NULL,
    ALTER COLUMN kind DROP DEFAULT,
    ALTER COLUMN place_entity_id DROP NOT NULL,
    ADD CONSTRAINT investigation_attempt_kind_check CHECK (
        kind IN ('entity_at_position', 'connected_place')
    ),
    ADD CONSTRAINT investigation_attempt_position_fkey
        FOREIGN KEY (character_entity_id, position_activity_id)
        REFERENCES position_version(entity_id, activity_id) ON DELETE RESTRICT;

-- Include the newly bound immutable identity in the existing lifecycle guard.
CREATE OR REPLACE FUNCTION protect_investigation_attempt() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'investigation attempt history is immutable'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'investigation_attempt_immutable_check';
    END IF;
    IF NEW.id <> OLD.id
       OR NEW.requested_by_user_id <> OLD.requested_by_user_id
       OR NEW.request_id <> OLD.request_id
       OR NEW.character_entity_id <> OLD.character_entity_id
       OR NEW.kind <> OLD.kind
       OR NEW.position_activity_id <> OLD.position_activity_id
       OR NEW.place_entity_id IS DISTINCT FROM OLD.place_entity_id
       OR NEW.outcome <> OLD.outcome
       OR NEW.created_at <> OLD.created_at
       OR (OLD.consumed_by_activity_id IS NOT NULL
           AND NEW.consumed_by_activity_id IS DISTINCT FROM OLD.consumed_by_activity_id)
       OR (OLD.voided_by_attempt_id IS NOT NULL
           AND NEW.voided_by_attempt_id IS DISTINCT FROM OLD.voided_by_attempt_id)
    THEN
        RAISE EXCEPTION 'investigation attempt identity and accepted lifecycle are immutable'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'investigation_attempt_immutable_check';
    END IF;
    RETURN NEW;
END;
$$;
