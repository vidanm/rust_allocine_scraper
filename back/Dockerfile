FROM rust:1.76-alpine
WORKDIR /home/vidan/Documents/Personnel/rust-allocine-parser/rust
COPY . .
ENV ROCKET_ADDRESS "127.0.0.1"
ENV ROCKET_PORT 8000
EXPOSE 8000
RUN apk add musl-dev libressl-dev && \
    cargo install --path .
CMD ["rust-allocine-parser"]

# RUN set-cont-env APP_NAME "Mozilla Firefox"
# RUN apk add --no-cache --upgrade bash musl-dev libressl-dev libx11
# CMD ["./gecko_cargo_wrapper.sh"]