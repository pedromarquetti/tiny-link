FROM rustlang/rust:nightly

RUN apt-get update -y && apt-get install -y vim

WORKDIR /var/www/tiny-link/
COPY . .

RUN cargo install --path .

CMD ["tiny-link"]
