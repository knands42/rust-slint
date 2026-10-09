CREATE TABLE sample
(
    id             TEXT NOT NULL PRIMARY KEY,
    created_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at     TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    
    CONSTRAINT sample_id_is_uuid CHECK (length(id) = 36)
);

CREATE TRIGGER sample_updated_at
    AFTER UPDATE ON sample
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE sample
    SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
    WHERE id = OLD.id;
END;