-- migrate:up

ALTER TABLE files ADD COLUMN folder_id INTEGER;

ALTER TABLE files ADD FOREIGN KEY (folder_id) REFERENCES folders(id);

-- migrate:down

ALTER TABLE files DROP COLUMN folder_id;

