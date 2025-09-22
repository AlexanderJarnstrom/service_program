// @generated automatically by Diesel CLI.

diesel::table! {
    customers (customer_id) {
        customer_id -> Uuid,
        f_name -> Varchar,
        l_name -> Varchar,
        email -> Nullable<Varchar>,
    }
}
