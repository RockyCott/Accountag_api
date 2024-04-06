mod routes;

use crate::routes::create_routes;
use colored::Colorize;

const PORT: u16 = 3000;

pub async fn run_app() {
    let app = create_routes();
    let ip_port = format!("0.0.0.0:{}", PORT);
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(&ip_port).await.unwrap();
    println!("{}", format!("Listening on port: {}", PORT).green());
    axum::serve(listener, app).await.unwrap();
}
