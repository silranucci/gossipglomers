# Gossip Glomers

Solutions to the [Gossip Glomers](https://fly.io/dist-sys/) distributed systems challenges by fly.io, implemented in Rust.

The challenges use [Maelstrom](https://github.com/jepsen-io/maelstrom), a workbench for writing toy distributed systems and testing them against a suite of consistency models.

## Challenges

| # | Challenge | Status |
|---|-----------|--------|
| 1 | Echo | Done |
| 2 | Unique ID Generation | Done |
| 3a | Single-Node Broadcast | - |
| 3b | Multi-Node Broadcast | - |
| 3c | Fault Tolerant Broadcast | - |
| 3d | Efficient Broadcast (Part I) | - |
| 3e | Efficient Broadcast (Part II) | - |
| 4 | Grow-Only Counter | - |
| 5a | Single-Node Kafka-Style Log | - |
| 5b | Multi-Node Kafka-Style Log | - |
| 5c | Efficient Kafka-Style Log | - |
| 6a | Single-Node, Totally-Available Transactions | - |
| 6b | Totally-Available, Read Uncommitted Transactions | - |
| 6c | Totally-Available, Read Committed Transactions | - |

## Project Structure

Each challenge lives in its own crate under `src/`. The `maelstrom` crate provides the shared node infrastructure (stdin/stdout message loop, serialization, etc.).

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with Compose

No local Rust or Java installation required — everything runs inside the container.

## Running a Challenge

Build and start the container:

```bash
docker compose up -d --build
```

Open a shell inside the container:

```bash
docker compose exec maelstrom bash
```

Inside the container, build and test a challenge:

```bash
cargo build --bin echo
maelstrom test -w echo \
  --bin ./target/debug/echo \
  --node-count 1 \
  --time-limit 10
```

Replace `echo` and the `-w` workload flag with the appropriate challenge binary and workload. Your source is mounted into the container, so edits on the host are picked up immediately, just rebuild and re-test.

Stop the container when done:

```bash
docker compose down
```

## Viewing Test Results

After running a test, start the Maelstrom web UI from inside the container:

```bash
maelstrom serve
```

Then open `http://localhost:8080` in your browser. Port 8080 is already exposed by the Docker Compose setup.

## How Maelstrom Works

Maelstrom spawns one or more node processes and communicates with them over stdin/stdout using a line-delimited JSON protocol. Each message has the shape:

```json
{
  "src": "c1",
  "dest": "n1",
  "body": {
    "type": "echo",
    "msg_id": 1,
    "echo": "hello"
  }
}
```

Nodes read from stdin, process the message, and write replies to stdout. Maelstrom then checks whether the system's behavior satisfies the required consistency guarantees.
