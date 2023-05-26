CREATE TABLE results (
    id BINARY(16) PRIMARY KEY, -- unique id of this test
    run_id BINARY(16), -- unique id of the run (group of tests)
    url VARCHAR(100),
    total_count INT,
    concurrency INT,
    attempt_count INT,
    success_count INT,
    error_count INT,
    start_timestamp TIMESTAMP,
    finish_timestamp TIMESTAMP,
    elapsed_time_ms INT,
    req_per_sec FLOAT
);