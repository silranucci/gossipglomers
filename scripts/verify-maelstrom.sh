#!/usr/bin/env bash
# Verify the most recent Maelstrom run succeeded.
#
# Maelstrom exits non-zero on failure, but we also parse store/latest/results.edn
# for the top-level :valid? flag as a belt-and-suspenders check — useful when the
# JVM exits cleanly but the checker found anomalies without a non-zero exit code.
#
# Usage: ./scripts/verify-maelstrom.sh
# Exit:  0 = valid, 1 = invalid or results not found

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RESULTS="$REPO_ROOT/store/latest/results.edn"

if [[ ! -f "$RESULTS" ]]; then
    echo "error: $RESULTS not found — did the test run complete?" >&2
    exit 1
fi

# The top-level :valid? key appears last in the file (after all sub-checker keys).
# We match only the final occurrence to avoid false positives from nested maps.
if grep -qE '^ *:valid\? true\}$' "$RESULTS"; then
    echo "maelstrom: run is valid"
    exit 0
else
    echo "maelstrom: run is INVALID" >&2
    echo "--- $RESULTS ---" >&2
    cat "$RESULTS" >&2
    exit 1
fi
