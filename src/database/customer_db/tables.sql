CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS customers (
  customer_id uuid DEFAULT uuid_generate_v4(),
  f_name VARCHAR NOT NULL,
  l_name VARCHAR NOT NULL, 
  email VARCHAR,
  PRIMARY KEY (customer_id)
);
