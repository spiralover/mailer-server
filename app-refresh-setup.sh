#!/bin/bash
echo "-> dropping database tables..."
diesel migration revert -a

sh app-setup.sh
