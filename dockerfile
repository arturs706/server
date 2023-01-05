FROM rust:1.66 as build


WORKDIR /src/actixlocal
COPY . .

RUN apt-get update && apt-get install -y \
    build-essential \
    libedit-dev \
    llvm \
    libclang-dev
RUN cargo build --release


RUN find / -name "libclang*" 2>/dev/null

    
FROM gcr.io/distroless/cc-debian10

COPY --from=build /src/actixlocal/target/release/actixlocal /usr/local/bin/actixlocal

WORKDIR /usr/local/bin

CMD ["actixlocal"]