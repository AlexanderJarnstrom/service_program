#!/bin/sh

set -e

if [[ ! -v SP_MACHINE_USER ]]; then
  echo "SP_MACHINE_USER is not set."
  exit 1
fi

if [[ ! -v SP_MACHINE_DATABASE ]]; then
  echo "SP_MACHINE_DATABASE is not set."
  exit 1
fi

if [[ ! -v SP_MACHINE_PASSWORD ]]; then
  echo "SP_MACHINE_PASSWORD is not set."
  exit 1
fi

if [[ ! -v POSTGRES_USER ]]; then
  echo "POSTGRES_USER is not set."
  exit 1
fi

echo "Creating user ${SP_MACHINE_USER}"
psql --username ${POSTGRES_USER} -c "CREATE USER ${SP_MACHINE_USER} WITH PASSWORD '$SP_MACHINE_PASSWORD';"

echo "Creating ${SP_MACHINE_DATABASE} for ${SP_MACHINE_USER}"
psql --username ${POSTGRES_USER} -c "CREATE DATABASE ${SP_MACHINE_DATABASE} OWNER ${SP_MACHINE_USER};"
