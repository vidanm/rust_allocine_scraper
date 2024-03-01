FROM rust:1.67
WORKDIR /usr/vidan/Documents/Personnel/rust-allocine-parser
COPY . .
RUN cargo install --path .
CMD ["rust-allocine-parser"]

# FROM alpine:3.14
# COPY geckodriver_install.sh geckodriver_install.sh
# #WORKDIR /usr/vidan/Documents/Personnel/rust-allocine-parser
# RUN apk add --no-cache --upgrade bash
# CMD ["./geckodriver_install.sh"]