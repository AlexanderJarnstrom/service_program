use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::machines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Machine {
    pub machine_id: i32,
    pub customer_id: uuid::Uuid,
    pub address: String,
    pub last_service: Option<NaiveDate>,
    pub next_service: Option<NaiveDate>
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::machines)]
pub struct InsertMachine {
    pub machine_id: Option<i32>,
    pub customer_id: uuid::Uuid,
    pub address: String,
    pub last_service: Option<NaiveDate>,
    pub next_service: Option<NaiveDate>
}
