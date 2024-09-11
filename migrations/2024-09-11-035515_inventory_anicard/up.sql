-- Your SQL goes here
CREATE TABLE inventory_anicard (
    id VARCHAR PRIMARY KEY,
    anicard_id INTEGER NOT NULL UNIQUE,
    inventory_id INTEGER NOT NULL,
    status_payment VARCHAR,
    left_amount INTEGER
)