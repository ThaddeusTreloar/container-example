#!/bin/sh

docker-compose exec kafka kafka-console-consumer.sh --topic log_sink --bootstrap-server kafka:9092 --from-beginning | rg $1 --line-buffered