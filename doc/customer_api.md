# Customer API

## Environment Variables

- **SP_DB_NAME**: name of database.
- **SP_DB_USER**: name of database owner.
- **SP_DB_PASS**: user password.
- **SP_DB_HOST**: database host.

## Libraries

- **warp**: 0.4.2 [crate](https://docs.rs/warp/latest/warp/)
- **uuid**: 1.18.1 [crate](https://docs.rs/uuid/1.18.1/uuid/)
- **tokio**: 1.47.1 [crate](https://docs.rs/tokio/latest/tokio/)
- **serde**: 1.0 [crate](https://docs.rs/serde/latest/serde/)
- **diesel**: 2.2.0 [crate](https://docs.rs/diesel/2.2.12/diesel/index.html)

## Calls

**get**:

```url
http://localhost:8000/api/customer
```

```url
http://localhost:8000/api/customer/<customer id>
```

**post**:

```url
http://localhost:8000/api/customer
```

```json
{
    "f_name": <fist name>,
    "l_name": <last name>,
    "email": <email/null>
}
```

**delete**:

```url
http://localhost:8000/api/customer/<customer_id>
```

**put*:

```url
http://localhost:8000/api/customer
```

```json
{
    "customer_id": <customer id>,
    "f_name": <fist name>,
    "l_name": <last name>,
    "email": <email/null>
}
```

