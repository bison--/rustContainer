FROM rust:1.59.0-buster

WORKDIR /coding/code

COPY compile.sh /usr/bin/compile
RUN chmod 777 /usr/bin/compile

COPY install.sh /usr/bin/install
RUN chmod 777 /usr/bin/install

SHELL ["/bin/bash", "-c"]
