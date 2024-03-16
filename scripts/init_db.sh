#! /usr/bin/env bash

set -x
set -eo pipefail

# Check that the dependencies are available
if ! [ -x "$(command -v psql)" ]; then
	echo >&s "Error: psql is not installed"
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed."
	exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

# Check if if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name has been, set otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a custom port has been set, otherwise default to '5454'
DB_PORT="${POSTGRES_PORT:=4545}"

# Check if a customhost has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# docker stop zero_to_prod_db

# Launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]; then
	docker run \
		-e POSTGRES_USER="${DB_USER}" \
		-e POSTGRES_PASSWORD="${DB_PASSWORD}" \
		-e POSTGRES_DB="${DB_NAME}" \
		-p "${DB_PORT}":5432 \
		-d postgres \
		postgres -N 1000 & # Increased number of connections for testing purposes
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	>&2 echo "Postgres is still unavailable - sleeping"
	sleep 5
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
