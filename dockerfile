FROM rust:slim

RUN set -ex
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN apt-get update
RUN apt-get install pkg-config -y --no-install-recommends
RUN apt-get install libssl-dev -y --no-install-recommends
RUN cargo install wasm-bindgen-cli
RUN mkdir /app

WORKDIR /app
COPY . .
CMD ["trunk", "serve", "--release"]
EXPOSE 8080