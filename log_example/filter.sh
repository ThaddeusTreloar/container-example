#!/bin/sh


query=".[] | select(.span.id | contains(\"${1}\"))"

cat log.json | jq -c "[.[] | select(.span != null)]" | jq -c "${query}" | jq