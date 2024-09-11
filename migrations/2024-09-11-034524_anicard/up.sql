-- Your SQL goes here
CREATE TABLE anicard (
    id VARCHAR PRIMARY KEY,
    character_name VARCHAR NOT NULL,
    favorites INTEGER NOT NULL,
    anime VARCHAR NOT NULL,
    img VARCHAR NOT NULL
)