# List available recipes
default:
    @just --list

# Build the Docker image (only needed once or after Dockerfile changes)
build-image:
    docker compose build

# Drop into a shell inside the container
shell:
    docker compose run --rm -it maelstrom bash

# Run a Maelstrom workload test.
#
# Usage:
#   just test echo               # 1 node, 10 s
#   just test echo 3             # 3 nodes, 10 s
#   just test echo 3 30          # 3 nodes, 30 s
test binary nodes="1" seconds="10":
    docker compose run --rm -it maelstrom bash -c \
        "cargo build --bin {{binary}} && \
         maelstrom test \
             -w {{binary}} \
             --bin target/debug/{{binary}} \
             --node-count {{nodes}} \
             --time-limit {{seconds}}"
