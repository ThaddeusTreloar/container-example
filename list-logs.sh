#!/bin/sh

docker-compose exec kafka kafka-console-consumer.sh --topic log_sink --bootstrap-server kafka:9092 --from-beginning | rg -o '"id":"[0-9A-F]+"' | rg -o '[0-9A-F]+' | sort -u
