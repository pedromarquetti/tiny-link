CREATE TABLE IF NOT EXISTS tiny_link (
    id SERIAL PRIMARY KEY,
    long_link TEXT NOT NULL,
    short_link VARCHAR(6) NOT NULL
)