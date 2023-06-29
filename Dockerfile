FROM rust:1.70

WORKDIR /usr/src/rust-antimat
COPY . .

RUN cargo install --path .

CMD ["rust-antimat"]