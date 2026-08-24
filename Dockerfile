# ---- builder ----
FROM rust:1-slim-bookworm AS builder
WORKDIR /build

# Copy just the manifest first and build a throwaway main.rs against it.
# This layer only re-runs when Cargo.toml/Cargo.lock change — so editing
# your own code later won't force axum/tokio/askama to recompile from
# scratch every time. Docker caches each layer; this trick exploits that.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now the real source. This layer reruns on every code change — but the
# dependency compilation above stays cached.
COPY src ./src
COPY templates ./templates
RUN touch src/main.rs && cargo build --release

# ---- runtime ----
FROM debian:bookworm-slim

# Running as root inside a container is a real attack surface: if this
# process is ever compromised, root-in-container plus a kernel bug is a
# path to root-on-host. useradd ships in bookworm-slim by default —
# no apt-get needed just to create a user.
RUN useradd --system --no-create-home appuser
USER appuser

COPY --from=builder /build/target/release/tadlock-pc-help /usr/local/bin/tadlock-pc-help

EXPOSE 8080
ENV RUST_LOG=info
CMD ["/usr/local/bin/tadlock-pc-help"]
