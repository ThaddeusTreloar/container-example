#!/bin/sh

set -f
query="{
  \"ksql\": \"SELECT * FROM trace_logs_unpart WHERE LOGID='$1';\",
  \"streamsProperties\": {
    \"auto.offset.reset\" : \"earliest\"
  }
}"

docker exec ksqldb-cli curl -w $'%{body}' -X "POST" "http://ksqldb-server:8088/query" \
     -H "Accept: application/vnd.ksql.v1+json" \
     -d "$query" -- | jq '.[] | "\(.header.schema) \(.row.columns)"' | hl