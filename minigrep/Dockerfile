FROM rust:1.54 as build

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /target/release/minigrep .
COPY ./poem.txt .


CMD ["./minigrep" , "to", "poem.txt"]
