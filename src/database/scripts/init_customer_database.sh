#!/bin/sh

set -e

if [[ ! -v SP_CUSTOMER_USER ]]; then
  echo "SP_CUSTOMER_USER is not set."
  exit 1
fi

if [[ ! -v SP_CUSTOMER_DATABASE ]]; then
  echo "SP_CUSTOMER_DATABASE is not set."
  exit 1
fi

if [[ ! -v SP_CUSTOMER_PASSWORD ]]; then
  echo "SP_CUSTOMER_PASSWORD is not set."
  exit 1
fi

if [[ ! -v POSTGRES_USER ]]; then
  echo "POSTGRES_USER is not set."
  exit 1
fi

echo "Creating user ${SP_CUSTOMER_USER} with ${SP_CUSTOMER_PASSWORD}"
psql --username ${POSTGRES_USER} -c "CREATE USER ${SP_CUSTOMER_USER} WITH PASSWORD '$SP_CUSTOMER_PASSWORD';"

echo "Creating ${SP_CUSTOMER_DATABASE} for ${SP_CUSTOMER_USER}"
psql --username ${POSTGRES_USER} -c "CREATE DATABASE ${SP_CUSTOMER_DATABASE} OWNER ${SP_CUSTOMER_USER};"

echo "Creating customer databases"
psql --username ${POSTGRES_USER} -d ${SP_CUSTOMER_DATABASE} -f ./customer_db/tables.sql
