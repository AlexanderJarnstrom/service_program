use crate::models::{Machine, InsertMachine};

use super::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_machines()
        .or(get_customer_machines())
        .or(delete_machine())
        .or(put_machine())
        .or(post_machine())
}

fn get_machines() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"]);

    warp::path!("api" / "machines")
        .and(warp::get())
        .and_then(handlers::get_machines)
        .with(cors)
}

fn get_customer_machines() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"]);

    warp::path!("api" / "machines" / String)
        .and(warp::get())
        .and_then(handlers::get_customer_machines)
        .with(cors)
}

fn delete_machine() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["DELETE"]);

    warp::path!("api" / "machines" / i32)
        .and(warp::delete())
        .and_then(handlers::delete_machine) 
        .with(cors)
}

fn put_json_body() -> impl Filter<Extract = (Machine,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn put_machine() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["PUT"]);

    warp::path!("api" / "machines")
        .and(warp::put())
        .and(put_json_body())
        .and_then(handlers::put_machine)
        .with(cors)
}

fn post_json_body() -> impl Filter<Extract = (InsertMachine,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}

fn post_machine() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let origin = "http://localhost";

    let cors = warp::cors()
        .allow_origin(origin)
        .allow_methods(vec!["POST"]);

    warp::path!("machines")
        .and(warp::post())
        .and(post_json_body())
        .and_then(handlers::post_machine)
        .with(cors)
}
