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
#   just test echo
#   just test echo 3 30
#   just test unique-ids 3 30 "--rate 1000 --availability total --nemesis partition"
test binary nodes="1" seconds="10" extra="":
    docker compose run --rm maelstrom bash -c \
        "cargo build --bin {{binary}} && \
         maelstrom test \
             -w {{binary}} \
             --bin target/debug/{{binary}} \
             --node-count {{nodes}} \
             --time-limit {{seconds}} \
             {{extra}}"
