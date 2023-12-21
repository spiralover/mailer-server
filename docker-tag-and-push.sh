#!/usr/bin/env bash

# Build Images
printf "Building & Tagging v%s & v%s \n\n" "$1" "$2";

echo "-> building and push user service..."
docker buildx build --platform linux/amd64,linux/arm64 \
  -t spiralover/mailer-user-service:"$1" \
  -t spiralover/mailer-user-service:"$2" \
  -t spiralover/mailer-user-service:latest . \
  -f apps/user/Dockerfile --push

echo "-> building and push executor service..."
docker buildx build --platform linux/amd64,linux/arm64 \
  -t spiralover/mailer-executor-service:"$1" \
  -t spiralover/mailer-executor-service:"$2" \
  -t spiralover/mailer-executor-service:latest . \
  -f apps/executor/Dockerfile --push
