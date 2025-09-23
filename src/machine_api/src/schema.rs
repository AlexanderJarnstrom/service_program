// @generated automatically by Diesel CLI.

diesel::table! {
    machines (machine_id) {
        machine_id -> Int4,
        customer_id -> Uuid,
        address -> Varchar,
        next_service -> Nullable<Date>,
        last_service -> Nullable<Date>,
    }
}
