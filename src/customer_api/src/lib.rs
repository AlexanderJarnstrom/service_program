mod models;
mod handlers;
mod routes;
mod database;
mod schema;

use crate::routes::routes;

#[tokio::main]
pub async fn run() {
    println!("Server started at http://localhost:8000");
    let routes = routes();
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await; 
}
