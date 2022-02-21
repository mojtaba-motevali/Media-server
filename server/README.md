## Description

This is a media server that scales vertically when needed.

## Installation

Requirements to install this application:

- rustc 1.58.1
- mediasoup's dependancies

Run `cargo install` to install dependancies.

## Running the app

You should set environment variables in .env file using .env.example as an example.

Then run `cargo run` to run the server.

### Production build:

run `cargo build` then pick up the binary generated under `/target/release/`

You're all done!

## References

https://actix.rs/

https://actix.rs/book/actix/sec-2-actor.html#:~:text=Actix%20is%20built%20on%20the,provided%20by%20the%20actix%20library.

https://mediasoup.org/
