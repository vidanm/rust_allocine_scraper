# FROM rust:1.76-alpine
# VOLUME /tmp/.X11-unix:/tmp/.X11-unix
# WORKDIR /usr/vidan/Documents/Personnel/rust-allocine-parser
# COPY . .
# EXPOSE 8008
# ENV DISPLAY=$DISPLAY
# CMD ["./gecko_cargo_wrapper.sh"]

FROM jlesage/baseimage-gui:alpine-3.15-v4
# WORKDIR /usr/vidan/Documents/Personnel/rust-allocine-parser
# WORKDIR /home/vidan/.mozilla/firefox/profiles.ini
# COPY . /home/
ENV USER_ID 1000
ENV GROUP_ID 1000
USER root
# RUN apk add bash musl-dev libressl-dev libx11 rust rustup cargo xterm wget
RUN apk add wget pciutils mesa-gl mesa-egl ttf-dejavu ttf-liberation

RUN wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub &&\
    wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.30-r0/glibc-2.30-r0.apk &&\
    wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.30-r0/glibc-bin-2.30-r0.apk &&\
    apk add glibc-2.30-r0.apk &&\
    apk add glibc-bin-2.30-r0.apk &&\
    apk add firefox-esr &&\
    wget https://github.com/mozilla/geckodriver/releases/download/v0.34.0/geckodriver-v0.34.0-linux64.tar.gz &&\
    tar -zxf geckodriver-v0.34.0-linux64.tar.gz -C /usr/bin &&\
    take-ownership /

COPY startapp.sh /startapp.sh
# RUN set-cont-env APP_NAME "Mozilla Firefox"
# RUN apk add --no-cache --upgrade bash musl-dev libressl-dev libx11
# CMD ["./gecko_cargo_wrapper.sh"]
