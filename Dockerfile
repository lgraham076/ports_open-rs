FROM rust:latest

RUN apt-get update && apt-get install -y \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-apple-darwin
RUN rustup target add x86_64-pc-windows-gnu

RUN useradd -ms /bin/bash rust-builder

USER rust-builder
WORKDIR /home/rust-builder

CMD ["/bin/bash"]