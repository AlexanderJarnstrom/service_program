use crate::{database, models::{Machine, InsertMachine}};

pub async fn get_machines() -> Result<impl warp::Reply, warp::Rejection> {
    // Get machines

    println!("Requested machine data");

    let machines = database::get_machines();

    Ok(warp::reply::json(&machines))
}

pub async fn get_customer_machines(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Get machines

    println!("Requested machine data for user {id}");

    let cid = uuid::Uuid::try_parse(&id)
        .expect("Could not parse");

    let machines = database::get_customer_machines(cid);

    Ok(warp::reply::json(&machines))
}

pub async fn delete_machine(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    // Delete machine

    println!("Deleting machine {}", id);

    database::delete_machine(id);

    Ok(warp::reply::reply())
}

pub async fn put_machine(machine: Machine) -> Result<impl warp::Reply, warp::Rejection> {
    // Update machine data
    
    println!("Updating machine");

    database::update_machine(machine);

    Ok(warp::reply::reply())
}

pub async fn post_machine(machine: InsertMachine) -> Result<impl warp::Reply, warp::Rejection> {
    // Add machine data

    println!("Adding machine");

    database::insert_machine(machine);

    Ok(warp::reply::reply())
}

