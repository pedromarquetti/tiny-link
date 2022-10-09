# Welcome to my link shortener server written in Rust

A simple server that recieves a long url as parameter and returns a tiny-url (that can be expanded)

## Running the server locally

### Docker

There's a Docker-compose file that can be run with `docker-compose up`

### Cargo

You can also do `RUST_LOG="info" cargo run`

## Links I used to learn

[Idea from here](https://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/)

[Setting up Diesel - with .env](https://diesel.rs/guides/getting-started)

[Basic `psql` setup on ubuntu](https://www.cherryservers.com/blog/how-to-install-and-setup-postgresql-server-on-ubuntu-20-04)

[URL Parsing - implemented in `parser.rs`](https://www.secretfader.com/blog/2019/01/parsing-validating-assembling-urls-rust/)
