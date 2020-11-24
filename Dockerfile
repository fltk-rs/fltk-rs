FROM alpine:3.11
COPY . .
RUN apk add rust cargo git cmake make g++ pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev
RUN cargo build
# RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --example editor
CMD ["echo", "Done building!"]
