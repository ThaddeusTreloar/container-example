#!/bin/bash

# Build app and mesh containers
docker build . -f ./init.Dockerfile  --tag init:latest
docker build . -f ./logging-processor.Dockerfile  --tag logging_processor:latest
docker build ./fake --tag fake-logger:latest

# Start containers
docker-compose up -d

# Setup kafka topics
docker-compose exec kafka kafka-topics.sh --create --topic log_sink --partitions 1 --replication-factor 1 --bootstrap-server kafka:9092

docker-compose exec kafka kafka-console-consumer.sh --topic log_sink --bootstrap-server kafka:9092 --from-beginning