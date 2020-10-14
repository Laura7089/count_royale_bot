#!/bin/sh
docker login -u laura7089 -p "$DOCKER_PASSWORD" 
docker push laura7089/countnite
