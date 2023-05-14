CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    user_name TEXT NOT NULL,
    user_role TEXT NOT NULL,
    user_pwd TEXT NOT NULL
)