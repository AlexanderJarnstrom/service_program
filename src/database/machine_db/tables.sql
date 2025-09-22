CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS machines (
  machine_id SERIAL,
  customer_id uuid NOT NULL,
  address VARCHAR NOT NULL,
  next_service DATE,
  last_service DATE,
  PRIMARY KEY (machine_id)
);
