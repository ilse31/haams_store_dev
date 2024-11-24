use dotenv::dotenv;
use haams_store_dev::app::App;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenv().ok();
    cli::main::<App, Migrator>().await
}
