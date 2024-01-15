#!/usr/bin/env bash

# Build Images: linux/amd64,linux/arm64
printf "Building & Tagging %s -> v%s & v%s \n\n" "$1" "$2" "$3";

echo "-> building and push user service..."
docker buildx build --platform "$1" \
  -t spiralover/mailer-user-service:"$2" \
  -t spiralover/mailer-user-service:"$3" \
  -t spiralover/mailer-user-service:latest . \
  -f apps/user/Dockerfile --push

echo "-> building and push executor service..."
docker buildx build --platform "$1" \
  -t spiralover/mailer-executor-service:"$2" \
  -t spiralover/mailer-executor-service:"$3" \
  -t spiralover/mailer-executor-service:latest . \
  -f apps/executor/Dockerfile --push
