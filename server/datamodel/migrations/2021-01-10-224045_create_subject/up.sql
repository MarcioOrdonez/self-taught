-- Your SQL goes here
create TABLE subject (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    status VARCHAR,
    creation_time timestamptz NOT NULL DEFAULT now(),
    deactivation_time timestamptz DEFAULT NULL,
    check_points INTEGER NOT NULL
)