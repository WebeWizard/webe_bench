#!/bin/bash


if [ -n "$ENVIRONMENT" ]; then
  cd tools/docker/mysql
    docker compose --env-file ../../../.env.${ENVIRONMENT} up -d
else
    echo "ENVIRONMENT not set"
fi

