FROM rust:1.65 as build


WORKDIR /src/axumdocker
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /src/axumdocker/target/release/dockerserver /usr/local/bin/axumdocker

WORKDIR /usr/local/bin

CMD ["axumdocker"]