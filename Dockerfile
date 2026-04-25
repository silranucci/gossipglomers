FROM rust:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-21-jre-headless \
    wget \
    gnuplot \
    graphviz \
    && rm -rf /var/lib/apt/lists/*

ARG MAELSTROM_VERSION=0.2.3
RUN wget -q "https://github.com/jepsen-io/maelstrom/releases/download/v${MAELSTROM_VERSION}/maelstrom.tar.bz2" \
    -O /tmp/maelstrom.tar.bz2 \
    && tar -xjf /tmp/maelstrom.tar.bz2 -C /opt \
    && rm /tmp/maelstrom.tar.bz2

ENV PATH="/opt/maelstrom:$PATH"

WORKDIR /app
