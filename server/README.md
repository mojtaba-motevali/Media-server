## Description

This is a media server that scales vertically when needed.

## Installation

Requirements to install this application:

- rustc 1.58.1
- mediasoup's dependancies

Run `cargo install` to install dependancies.

### Running the app

You should set environment variables in .env file using .env.example as an example.

Then run `cargo run` to run the server.

### Production

run `cargo build` then pick up the binary generated under `/target/release/`

You're all done!

## References

https://actix.rs/

https://actix.rs/book/actix/

https://mediasoup.org/
