-- Your SQL goes here
CREATE TABLE ingredients (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name varchar NOT NULL UNIQUE,
    description text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp)
