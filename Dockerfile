FROM rust:1.67

WORKDIR /usr/vidan/Documents/Personnel/rust-allocine-parser
COPY . .

RUN cargo install --path .

CMD ["rust-allocine-parser"]
