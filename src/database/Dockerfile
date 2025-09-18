FROM postgres:latest
COPY scripts/init_customer_database.sh /docker-entrypoint-initdb.d/10-init_customer_database.sh
COPY scripts/init_machine_database.sh /docker-entrypoint-initdb.d/20-init_machine_database.sh
