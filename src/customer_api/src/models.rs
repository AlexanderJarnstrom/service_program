use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub customer_id: uuid::Uuid,
    pub l_name: String,
    pub f_name: String,
    pub email: Option<String>
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name = crate::schema::customers)]
pub struct InsertCustomer {
    pub customer_id: Option<uuid::Uuid>,
    pub l_name: String,
    pub f_name: String,
    pub email: Option<String>
}
