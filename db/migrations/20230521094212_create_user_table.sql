-- migrate:up
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);

INSERT INTO users (name, email, password)
VALUES (
    'Admin',
    'support@goworkwize.com',
    crypt('password', gen_salt('bf', 8))
);

-- migrate:down
DROP TABLE users;
DROP EXTENSION pgcrypto;