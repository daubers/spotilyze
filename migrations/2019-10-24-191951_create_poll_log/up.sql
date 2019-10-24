CREATE TABLE poll_log (
    id SERIAL PRIMARY KEY,
    insert_time TIMESTAMP DEFAULT now(),
    uri VARCHAR NOT NULL,
    sent_data JSON,
    return_code INTEGER
);