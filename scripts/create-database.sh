#!/bin/bash

set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER demo WITH PASSWORD 'password';
    CREATE DATABASE reddit-db-layer;
    GRANT ALL PRIVILEGES ON DATABASE reddit-db-layer TO demo;
    \c reddit-db-layer
    GRANT ALL ON SCHEMA public TO demo;
EOSQL