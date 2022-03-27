FROM rust:1.59.0-buster

WORKDIR /coding/code

COPY compile.sh /usr/bin/compile
RUN chmod 777 /usr/bin/compile

COPY compile_windows.sh /usr/bin/compile-win
RUN chmod 777 /usr/bin/compile-win

COPY install.sh /usr/bin/install
RUN chmod 777 /usr/bin/install


# cross-compile windows
RUN apt update && apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu


SHELL ["/bin/bash", "-c"]
