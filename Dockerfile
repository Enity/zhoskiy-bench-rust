# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.


# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `using-sqlx`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/zhoskiy-bench-rust \
    /usr/local/bin/
CMD /usr/local/bin/zhoskiy-bench-rust