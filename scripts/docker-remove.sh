#!/bin/bash

if [ -n "$ENVIRONMENT" ]; then
    cd tools/docker/mysql
    docker compose --env-file ../../../.env.${ENVIRONMENT} down --volumes --rmi all
else
    echo "ENVIRONMENT not set"
fi