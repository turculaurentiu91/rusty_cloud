-- migrate:up

CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    file_name TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    size_in_bytes BIGINT NOT NULL CHECK (size_in_bytes >= 0),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

-- Create an index on the owner_id column for faster lookups
CREATE INDEX idx_files_owner_id ON files (owner_id);

-- migrate:down

DROP TABLE files;

