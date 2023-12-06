#!/bin/sh

query=".[].span.id"

cat log.json | jq -c "[.[] | select(.span != null)]" | jq -c ".[].span.id" | jq -r | sort | uniq
