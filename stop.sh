#!/bin/bash

docker-compose down

docker image prune
docker volume prune -a