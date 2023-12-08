#!/bin/bash

# Build app and mesh containers
docker build . -f ./dockerfiles/init.Dockerfile  --tag init:latest --no-cache
docker build . -f ./dockerfiles/logging_processor.Dockerfile  --tag logging_processor:latest --no-cache
docker build . -f ./dockerfiles/entity_microservice.Dockerfile --tag entity_microservice:latest --no-cache
docker build . -f ./dockerfiles/property_microservice.Dockerfile --tag property_microservice:latest --no-cache
docker build . -f ./dockerfiles/combo_service.Dockerfile --tag combo_service:latest --no-cache
docker build . -f ./dockerfiles/proxy_handler.Dockerfile --tag proxy_handler:latest --no-cache

# Start containers
docker-compose up -d

# Setup kafka topics
docker-compose exec kafka kafka-topics.sh --create --topic log_sink --partitions 1 --replication-factor 1 --bootstrap-server kafka:9092

# Setup ksqldb
cat log_process.ksql | docker exec -i ksqldb-cli ksql http://ksqldb-server:8088