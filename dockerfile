FROM rust:1.65 as build


WORKDIR /src/actixlocal
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /src/actixlocal/target/release/dockerserver /usr/local/bin/actixlocal

WORKDIR /usr/local/bin

CMD ["actixlocal"]