CREATE TABLE character (
    entity_id uuid NOT NULL,
    owner_user_id uuid NOT NULL,
    CONSTRAINT character_pkey PRIMARY KEY (entity_id),
    CONSTRAINT character_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity(id) ON DELETE RESTRICT,
    CONSTRAINT character_owner_user_id_key UNIQUE (owner_user_id),
    CONSTRAINT character_owner_user_id_fkey
        FOREIGN KEY (owner_user_id) REFERENCES "user"(id) ON DELETE RESTRICT
);
