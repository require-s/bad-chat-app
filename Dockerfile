FROM alpine:3.21

# I DON'T UNDERSTAND WHY THE COMMENTED OUT BITS DOESN'T WORK

# RUN wget -q "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-musl/rustup-init"; \
#     chmod +xr ./rustup-init; \
#     # cat ./rustup-init; \
#     ./rustup-init -y --no-modify-path --default-toolchain stable;  \
#     rm ./rustup-init;

# ENV PATH=${PATH}:~/.cargo/bin
# RUN set -eux; \
#     echo $(ls -1 ~); \
#     rustup --version;

RUN apk add rust cargo

WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.lock ./

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN cargo build --release;
CMD [ "./target/release/badchatapp" ]