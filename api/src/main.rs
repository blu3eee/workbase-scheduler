use dotenv::dotenv;

use api::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = run().await;
}
