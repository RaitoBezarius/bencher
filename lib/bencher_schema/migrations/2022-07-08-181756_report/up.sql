-- https://sqlite.org/datatype3.html#storage_classes_and_datatypes
-- https://www.sqlite.org/autoinc.html
CREATE TABLE user (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    admin BOOLEAN NOT NULL,
    locked BOOLEAN NOT NULL
);
CREATE TABLE token (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    jwt TEXT NOT NULL,
    creation BIGINT NOT NULL,
    expiration BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id)
);
CREATE TABLE organization (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE
);
CREATE TABLE organization_role (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    role TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (organization_id) REFERENCES organization (id),
    UNIQUE(user_id, organization_id)
);
CREATE TABLE project (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    organization_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    url TEXT,
    public BOOLEAN NOT NULL,
    FOREIGN KEY (organization_id) REFERENCES organization (id)
);
CREATE TABLE project_role (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    role TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (project_id) REFERENCES project (id),
    UNIQUE(user_id, project_id)
);
CREATE TABLE branch (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project (id),
    UNIQUE(project_id, slug)
);
CREATE TABLE version (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    branch_id INTEGER NOT NULL,
    number INTEGER NOT NULL,
    hash TEXT,
    FOREIGN KEY (branch_id) REFERENCES branch (id),
    UNIQUE(branch_id, number)
);
CREATE TABLE testbed (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    os_name TEXT,
    os_version TEXT,
    runtime_name TEXT,
    runtime_version TEXT,
    cpu TEXT,
    ram TEXT,
    disk TEXT,
    FOREIGN KEY (project_id) REFERENCES project (id),
    UNIQUE(project_id, slug)
);
CREATE TABLE benchmark (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project (id),
    UNIQUE(project_id, name)
);
CREATE TABLE report (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    version_id INTEGER NOT NULL,
    testbed_id INTEGER NOT NULL,
    adapter INTEGER NOT NULL,
    start_time BIGINT NOT NULL,
    end_time BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (version_id) REFERENCES version (id),
    FOREIGN KEY (testbed_id) REFERENCES testbed (id)
);
CREATE TABLE latency (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    lower_variance BIGINT NOT NULL,
    upper_variance BIGINT NOT NULL,
    duration BIGINT NOT NULL
);
CREATE TABLE throughput (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    lower_variance DOUBLE NOT NULL,
    upper_variance DOUBLE NOT NULL,
    events DOUBLE NOT NULL,
    unit_time BIGINT NOT NULL
);
CREATE TABLE resource (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    min DOUBLE NOT NULL,
    max DOUBLE NOT NULL,
    avg DOUBLE NOT NULL
);
CREATE TABLE perf (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    report_id INTEGER NOT NULL,
    iteration INTEGER NOT NULL,
    benchmark_id INTEGER NOT NULL,
    -- at least one should not be null
    latency_id INTEGER,
    throughput_id INTEGER,
    compute_id INTEGER,
    memory_id INTEGER,
    storage_id INTEGER,
    --
    FOREIGN KEY (report_id) REFERENCES report (id),
    FOREIGN KEY (benchmark_id) REFERENCES benchmark (id),
    FOREIGN KEY (latency_id) REFERENCES latency (id),
    FOREIGN KEY (throughput_id) REFERENCES throughput (id),
    FOREIGN KEY (compute_id) REFERENCES resource (id),
    FOREIGN KEY (memory_id) REFERENCES resource (id),
    FOREIGN KEY (storage_id) REFERENCES resource (id),
    UNIQUE(report_id, iteration, benchmark_id)
);
-- https://en.wikipedia.org/wiki/Standard_score
-- https://en.wikipedia.org/wiki/Student's_t-test
CREATE TABLE statistic (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    -- test kind: Z or T
    test INTEGER NOT NULL,
    -- sample size
    sample_size BIGINT,
    -- time window
    window BIGINT,
    -- left side percentage
    left_side REAL,
    -- right side percentage
    right_side REAL
);
CREATE TABLE threshold (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    branch_id INTEGER NOT NULL,
    testbed_id INTEGER NOT NULL,
    kind INTEGER NOT NULL,
    statistic_id INTEGER NOT NULL,
    FOREIGN KEY (branch_id) REFERENCES branch (id),
    FOREIGN KEY (testbed_id) REFERENCES testbed (id),
    FOREIGN KEY (statistic_id) REFERENCES statistic (id),
    UNIQUE(branch_id, testbed_id, kind)
);
CREATE TABLE alert (
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    perf_id INTEGER NOT NULL,
    threshold_id INTEGER NOT NULL,
    statistic_id INTEGER NOT NULL,
    side BOOLEAN NOT NULL,
    boundary REAL NOT NULL,
    outlier REAL NOT NULL,
    FOREIGN KEY (perf_id) REFERENCES perf (id),
    FOREIGN KEY (threshold_id) REFERENCES threshold (id),
    FOREIGN KEY (statistic_id) REFERENCES statistic (id)
);