-- Your SQL goes here
-- Your SQL goes here
create TABLE check_point (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    status VARCHAR,
    creation_time timestamptz NOT NULL DEFAULT now(),
    deactivation_time timestamptz DEFAULT NULL,
    precedence INTEGER NOT NULL,
    subject_id INTEGER NOT NULL,
    FOREIGN KEY (subject_id) references subject(id) ON DELETE CASCADE
)
