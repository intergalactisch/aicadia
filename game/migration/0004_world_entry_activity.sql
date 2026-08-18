CREATE TABLE place (
    entity_id uuid NOT NULL,
    is_entry boolean NOT NULL,
    CONSTRAINT place_pkey PRIMARY KEY (entity_id),
    CONSTRAINT place_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT
);

CREATE UNIQUE INDEX place_one_entry_index ON place (is_entry) WHERE is_entry;

ALTER TABLE character
    ADD COLUMN current_place_entity_id uuid,
    ADD CONSTRAINT character_current_place_entity_id_fkey
        FOREIGN KEY (current_place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT;

CREATE TABLE activity (
    id uuid PRIMARY KEY,
    operation text NOT NULL,
    requested_by_user_id uuid NOT NULL,
    actor_character_entity_id uuid,
    context_place_entity_id uuid,
    occurred_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT activity_operation_check CHECK (
        operation IN ('create_character', 'create_entity', 'create_entry_place', 'enter_world')
    ),
    CONSTRAINT activity_requested_by_user_id_fkey
        FOREIGN KEY (requested_by_user_id) REFERENCES "user"(id) ON DELETE RESTRICT,
    CONSTRAINT activity_actor_character_entity_id_fkey
        FOREIGN KEY (actor_character_entity_id) REFERENCES character(entity_id) ON DELETE RESTRICT,
    CONSTRAINT activity_context_place_entity_id_fkey
        FOREIGN KEY (context_place_entity_id) REFERENCES place(entity_id) ON DELETE RESTRICT
);

CREATE TABLE activity_entity (
    activity_id uuid NOT NULL,
    entity_id uuid NOT NULL,
    role text NOT NULL,
    CONSTRAINT activity_entity_pkey PRIMARY KEY (activity_id, entity_id, role),
    CONSTRAINT activity_entity_role_check CHECK (role IN ('subject', 'destination')),
    CONSTRAINT activity_entity_activity_id_fkey
        FOREIGN KEY (activity_id) REFERENCES activity(id) ON DELETE RESTRICT,
    CONSTRAINT activity_entity_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT
);

CREATE INDEX activity_actor_occurred_at_id_index
    ON activity (actor_character_entity_id, occurred_at DESC, id DESC)
    WHERE actor_character_entity_id IS NOT NULL;
CREATE INDEX activity_entity_entity_id_activity_id_index
    ON activity_entity (entity_id, activity_id);

-- Before this migration every Character Entity can only have come from
-- create_character, and every Entity without a Character role can only have come
-- from create_entity. Those operation, requester, subject and timestamp facts are
-- therefore exact. The old schema did not retain actor or Place context, so this
-- backfill deliberately leaves both absent.
INSERT INTO activity (
    id,
    operation,
    requested_by_user_id,
    actor_character_entity_id,
    context_place_entity_id,
    occurred_at
)
SELECT (
           substr(md5('activity:' || entity.id::text), 1, 8) || '-' ||
           substr(md5('activity:' || entity.id::text), 9, 4) || '-' ||
           substr(md5('activity:' || entity.id::text), 13, 4) || '-' ||
           substr(md5('activity:' || entity.id::text), 17, 4) || '-' ||
           substr(md5('activity:' || entity.id::text), 21, 12)
       )::uuid,
       CASE WHEN character.entity_id IS NULL THEN 'create_entity' ELSE 'create_character' END,
       CASE
           WHEN character.entity_id IS NULL THEN entity.introduced_by_user_id
           ELSE character.owner_user_id
       END,
       NULL,
       NULL,
       entity.introduced_at
FROM entity
LEFT JOIN character ON character.entity_id = entity.id;

INSERT INTO activity_entity (activity_id, entity_id, role)
SELECT activity.id, entity.id, 'subject'
FROM entity
JOIN activity
  ON activity.id = (
         substr(md5('activity:' || entity.id::text), 1, 8) || '-' ||
         substr(md5('activity:' || entity.id::text), 9, 4) || '-' ||
         substr(md5('activity:' || entity.id::text), 13, 4) || '-' ||
         substr(md5('activity:' || entity.id::text), 17, 4) || '-' ||
         substr(md5('activity:' || entity.id::text), 21, 12)
     )::uuid;

CREATE FUNCTION reject_activity_change() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'activity history is immutable';
END;
$$;

CREATE TRIGGER activity_immutable
    BEFORE UPDATE OR DELETE ON activity
    FOR EACH ROW EXECUTE FUNCTION reject_activity_change();
CREATE TRIGGER activity_entity_immutable
    BEFORE UPDATE OR DELETE ON activity_entity
    FOR EACH ROW EXECUTE FUNCTION reject_activity_change();
