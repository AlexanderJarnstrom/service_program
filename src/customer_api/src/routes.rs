use crate::models::{Customer, InsertCustomer};

use super::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_customers()
        .or(delete_customers())
        .or(put_customer())
        .or(post_customer())
}

fn get_customers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("GET");

    warp::path!("api" / "customers")
        .and(warp::get())
        .and_then(handlers::get_customers)
        .with(cors)
}

fn delete_customers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("DELETE");

    warp::path!("api" / "customers" / String)
        .and(warp::delete())
        .and_then(handlers::delete_customer) 
        .with(cors)
}

fn put_json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn put_customer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("PUT");

    warp::path!("api" / "customers")
        .and(warp::put())
        .and(put_json_body())
        .and_then(handlers::put_customer)
        .with(cors)
}

fn post_json_body() -> impl Filter<Extract = (InsertCustomer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn post_customer() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("POST");

    warp::path!("api" / "customers")
        .and(warp::post())
        .and(post_json_body())
        .and_then(handlers::post_customer)
        .with(cors)
}
