-- migrate:up

CREATE TABLE folders (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id INTEGER,
  owner_id INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),

  FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (parent_id) REFERENCES folders(id)
);

-- migrate:down

DROP TABLE folders;
