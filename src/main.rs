use accountag_api::run_app;

#[tokio::main]
async fn main() {
    // build our application
    run_app().await;

}
