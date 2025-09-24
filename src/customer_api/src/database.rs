use diesel::{prelude::*, serialize::ToSql};
use crate::models::*;

use std::env;

fn connect_db() -> PgConnection {
    let db_name = env::var("SP_DB_NAME").expect("SP_DB_NAME must be defined.");
    let db_user = env::var("SP_DB_USER").expect("SP_DB_USER must be defined.");
    let db_password = env::var("SP_DB_PASS").expect("SP_DB_PASS must be defined.");
    let db_host = env::var("SP_DB_HOST").expect("SP_DB_HOST must be defined.");

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}/{db_name}");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error: could not connect to {}", db_url))
}

pub fn get_customers() -> Vec<Customer> {
    use crate::schema::customers::dsl::*;
    let db_con = &mut connect_db();
    let res = customers
        .select(Customer::as_select())
        .load(db_con)
        .expect("Error loading customers");

    return res;
}

pub fn get_customer(cid: uuid::Uuid) -> Customer {
    use crate::schema::customers::dsl::*;

    let db_conn = &mut connect_db();
    let res = customers
        .select(Customer::as_select())
        .filter(customer_id.eq(cid))
        .first(db_conn)
        .expect("Error loading customers");

    return res;
}

pub fn delete_customer(cid: uuid::Uuid) {
    use crate::schema::customers::dsl::*;

    let db_conn = &mut connect_db();

    diesel::delete(customers.find(cid))
        .execute(db_conn)
        .expect("Error removing customer");
}

pub fn update_customer(customer: Customer) {
    use crate::schema::customers::dsl::*;

    let db_conn = &mut connect_db();
    diesel::update(customers.find(customer.customer_id))
        .set((
            f_name.eq(customer.f_name),
            l_name.eq(customer.l_name),
            email.eq(customer.email)
        ))
        .execute(db_conn)
        .expect("Failed to update");
}

pub fn insert_customer(customer: InsertCustomer) {
    use crate::schema::customers;

    let db_conn = &mut connect_db(); 
    diesel::insert_into(customers::table)
        .values(&customer)
        .returning(Customer::as_returning())
        .get_result(db_conn)
        .expect("Error inserting customer.");
}

