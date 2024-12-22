#[cfg(test)]
mod tests;

use anyhow::Result;
use dotenv::dotenv;
use routes::Router;
use server::start_server;

mod controllers;
mod db;
mod routes;
mod server;
mod views;
mod macros;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    db::setup_database()?;
    db::create_users_table()?;
    db::insert_user_from_env()?;
    db::create_contact_table()?;
    db::create_campaigns_table()?;
    db::create_goals_table()?;
    //db::create_donation_crypto_table()?;
    db::create_campaign_updates_table()?;

    let router = Router::new();
    let app = router.create_router();
    start_server(app).await;

    Ok(())
}
