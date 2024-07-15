FROM alpine:3.14
ENV USER_ID=1000
ENV GROUP_ID=1000
USER root
EXPOSE 4444
RUN apk add wget pciutils mesa-gl mesa-egl ttf-dejavu ttf-liberation
RUN wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub &&\
    wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.30-r0/glibc-2.30-r0.apk &&\
    wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.30-r0/glibc-bin-2.30-r0.apk &&\
    apk add glibc-2.30-r0.apk &&\
    apk add glibc-bin-2.30-r0.apk &&\
    apk add firefox &&\
    wget https://github.com/mozilla/geckodriver/releases/download/v0.34.0/geckodriver-v0.34.0-linux64.tar.gz &&\
    tar -zxf geckodriver-v0.34.0-linux64.tar.gz -C /usr/bin
COPY startapp.sh /startapp.sh
ENTRYPOINT ["/startapp.sh"]
