CREATE TABLE app_user (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE entity (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    description text NOT NULL,
    introduced_by_user_id uuid NOT NULL REFERENCES app_user(id),
    introduced_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT entity_name_trimmed CHECK (name = btrim(name)),
    CONSTRAINT entity_name_length CHECK (char_length(name) BETWEEN 1 AND 120),
    CONSTRAINT entity_description_trimmed CHECK (description = btrim(description)),
    CONSTRAINT entity_description_length CHECK (char_length(description) BETWEEN 1 AND 4000)
);

CREATE INDEX entity_introduced_at_id_index ON entity (introduced_at DESC, id DESC);
