PRAGMA foreign_keys = off;
-- Deduplicate metric_kinds
DELETE FROM metric_kind
WHERE rowid NOT IN (
        SELECT min(rowid)
        FROM metric_kind
        GROUP BY project_id,
            name
    );
CREATE TABLE up_metric_kind (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    units TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE,
    UNIQUE(project_id, name),
    UNIQUE(project_id, slug)
);
INSERT INTO up_metric_kind(
        id,
        uuid,
        project_id,
        name,
        slug,
        units
    )
SELECT id,
    uuid,
    project_id,
    name,
    slug,
    units
FROM metric_kind;
DROP TABLE metric_kind;
ALTER TABLE up_metric_kind
    RENAME TO metric_kind;
PRAGMA foreign_keys = on;