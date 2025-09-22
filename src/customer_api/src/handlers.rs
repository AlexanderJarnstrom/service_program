use crate::{database, models::{Customer, InsertCustomer}};

pub async fn get_customers() -> Result<impl warp::Reply, warp::Rejection> {
    // Get customer data

    println!("Requested user data");

    let customers = database::get_customers();

    Ok(warp::reply::json(&customers))
}

pub async fn delete_customer(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Delete customer

    println!("Deleting customer {}", id);

    let cid = uuid::Uuid::try_parse(&id)
        .expect("Could not parse");

    database::delete_customer(cid);

    Ok(warp::reply::reply())
}

pub async fn put_customer(customer: Customer) -> Result<impl warp::Reply, warp::Rejection> {
    // Update customer data
    
    println!("Updating customer");

    database::update_customer(customer);

    Ok(warp::reply::reply())
}

pub async fn post_customer(customer: InsertCustomer) -> Result<impl warp::Reply, warp::Rejection> {
    // Add customer data

    println!("Adding customer");

    database::insert_customer(customer);

    Ok(warp::reply::reply())
}

