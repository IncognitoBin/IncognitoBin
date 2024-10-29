#!/bin/sh
# env.sh

find /usr/share/nginx/html -type f -name "*.js" -exec sed -i 's|%%API_URL%%|'${API_URL}'|g' {} +