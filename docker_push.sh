#!/bin/sh
echo "$DOCKER_PASSWORD" | docker login -u laura7089 -p
docker push laura7089/countnite
