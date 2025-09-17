mod models;
mod handlers;
mod routes;

use crate::routes::routes;

#[tokio::main]
pub async fn run() {
    let routes = routes();
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await; 
}
