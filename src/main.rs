// Uncomment this block to pass the first stage
use redis_starter_rust::app::App;

#[tokio::main]
async fn main() -> redis_starter_rust::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage
    //
    let mut app = App::new("127.0.0.1:6379", 100)?;
    app.run().await?;

    Ok(())
}

