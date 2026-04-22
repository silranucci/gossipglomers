FROM rust:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-21-jre-headless \
    curl \
    wget \
    gnuplot \
    graphviz \
    && rm -rf /var/lib/apt/lists/*

ARG MAELSTROM_VERSION=0.2.3
RUN wget -q "https://github.com/jepsen-io/maelstrom/releases/download/v${MAELSTROM_VERSION}/maelstrom.tar.bz2" \
    -O /tmp/maelstrom.tar.bz2 \
    && tar -xjf /tmp/maelstrom.tar.bz2 -C /opt \
    && rm /tmp/maelstrom.tar.bz2 \
    && echo 'export PATH="/opt/maelstrom:$PATH"' >> /etc/bash.bashrc

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --bins

ENTRYPOINT ["/bin/bash"]
