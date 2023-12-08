#/bin/sh

cat log_process.ksql | docker exec -i ksqldb-cli ksql http://ksqldb-server:8088