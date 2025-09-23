use diesel::prelude::*;
use crate::{models::*, schema::machines};

use std::env;

fn connect_db() -> PgConnection {
    let db_name = env::var("SP_DB_NAME").expect("SP_DB_NAME must be defined.");
    let db_user = env::var("SP_DB_USER").expect("SP_DB_USER must be defined.");
    let db_password = env::var("SP_DB_PASS").expect("SP_DB_PASS must be defined.");
    let db_host = env::var("SP_DB_HOST").expect("SP_DB_HOST must be defined.");

    let db_url = format!("postgres://{db_user}:{db_password}@{db_host}/{db_name}");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error: could not connect to {}", db_url))
}

pub fn get_machines() -> Vec<Machine> {
    use crate::schema::machines::dsl::*;
    let db_con = &mut connect_db();
    let res = machines
        .select(Machine::as_select())
        .load(db_con)
        .expect("Error loading machines");

    return res;
}

pub fn get_customer_machines(cid: uuid::Uuid) -> Vec<Machine> {
    use crate::schema::machines::dsl::*;
    use crate::schema::machines;

    let db_con = &mut connect_db();
    let res = machines
        .select(Machine::as_select())
        .filter(machines::customer_id.eq(cid))
        .load(db_con)
        .expect("Error loading machines");

    return res;
}

pub fn delete_machine(mid: i32) {
    use crate::schema::machines::dsl::*;

    let db_conn = &mut connect_db();

    diesel::delete(machines.find(mid))
        .execute(db_conn)
        .expect("Error removing machine");
}

pub fn update_machine(machine: Machine) {
    use crate::schema::machines::dsl::*;

    let db_conn = &mut connect_db();
    diesel::update(machines.find(machine.machine_id))
        .set((
            customer_id.eq(machine.customer_id),
            address.eq(machine.address),
            last_service.eq(machine.last_service),
            next_service.eq(machine.next_service),
        ))
        .execute(db_conn)
        .expect("Failed to update machine");
}

pub fn insert_machine(machine: InsertMachine) {
    use crate::schema::machines;

    let db_conn = &mut connect_db(); 
    diesel::insert_into(machines::table)
        .values(&machine)
        .execute(db_conn)
        .expect("Error inserting machine");
}

