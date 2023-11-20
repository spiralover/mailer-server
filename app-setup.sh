#!/bin/bash
echo "-> migrating database tables..."
diesel migration run
echo "-> seeding database..."
curl http://localhost:4301/system/database-seed
# shellcheck disable=SC2028
echo "\n-> misc setup..."
mkdir -p static/uploads
echo ""
