# Customer API

## Libraries

- **warp**: 0.4.2 [crate](https://docs.rs/warp/latest/warp/)
- **uuid**: 1.18.1 [crate](https://docs.rs/uuid/1.18.1/uuid/)
- **tokio**: 1.47.1 [crate](https://docs.rs/tokio/latest/tokio/)
- **serde**: 1.0 [crate](https://docs.rs/serde/latest/serde/)
- **diesel**: 2.2.0 [crate](https://docs.rs/diesel/2.2.12/diesel/index.html)

## Calls

**get**:

```url
http://localhost:8000/customer

```

**post**:

```url
http://localhost:8000/customer
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
http://localhost:8000/customer/<customer_id>
```

**put*:

```url
http://localhost:8000/customer
```

```json
{
    "customer_id": <customer id>,
    "f_name": <fist name>,
    "l_name": <last name>,
    "email": <email/null>
}
```

