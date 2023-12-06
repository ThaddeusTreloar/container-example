#!/bin/sh

query=".[].span.id"

cat log.json | jq -c "${query}" | jq -r | grep -v null | uniq
