use anyhow::Error;
use shared::init::start_server;

mod biz_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    start_server(biz_router::get_router()).await
}
