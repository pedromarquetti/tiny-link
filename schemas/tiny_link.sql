CREATE TABLE tiny_link (
    id SERIAL PRIMARY KEY,
    long_link TEXT NOT NULL,
    short_link VARCHAR(6) NOT NULL 
)  -- tut https://www.cherryservers.com/blog/how-to-install-and-setup-postgresql-server-on-ubuntu-20-04