# Database

## Environment Variables

The variables are located in:

```
./src/database/database.env
```

- **POSTGRES_PASSWORD**: admin password for database. (required)
- **SP_CUSTOMER_USER**: username for customer database manager. (required)
- **SP_CUSTOMER_PASSWORD**: password for the customer user. (required)
- **SP_CUSTOMER_DATABASE**: customer database name. (required)
- **SP_MACHINE_USER**: username for machine database manager. (required)
- **SP_MACHINE_PASSWORD**: password for the machine user. (required)
- **SP_MACHINE_DATABASE**: machine database name. (required)

## Tables

### Customers

- **cid**: customer id, *uuid*, primary key.
- **f_name**: customer first name, varchar, required.
- **l_name**: customer last name, varchar, required.
- **email**: customer email, varchar.

### Machines

- **mid**: machine id, *serial*, primary key.
- **cid**: customer id, *uuid*, required.
- **address**: machine address/location, *varchar*, required.
- **last_service**: last service date, *date*.
- **next_service**: next service date, *date*.
