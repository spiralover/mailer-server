#!/usr/bin/env bash

# Build Images
echo "-> building and push user service..."
docker buildx build --platform linux/amd64,linux/arm64 \
  -t spiralover/mailer-user-service:0.1 \
  -t spiralover/mailer-user-service:0.1.1 \
  -t spiralover/mailer-user-service:latest . \
  -f apps/user/Dockerfile --push

echo "-> building and push executor service..."
docker buildx build --platform linux/amd64,linux/arm64 \
  -t spiralover/mailer-executor-service:0.1 \
  -t spiralover/mailer-executor-service:0.1.1 \
  -t spiralover/mailer-executor-service:latest . \
  -f apps/executor/Dockerfile --push
