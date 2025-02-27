PRAGMA foreign_keys = off;
CREATE TABLE down_metric_kind (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    units TEXT,
    FOREIGN KEY (project_id) REFERENCES project (id),
    UNIQUE(project_id, slug)
);
INSERT INTO down_metric_kind(
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
ALTER TABLE down_metric_kind
    RENAME TO metric_kind;
PRAGMA foreign_keys = on;