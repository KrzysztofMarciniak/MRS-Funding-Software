use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn get_connection() -> Result<Connection, DatabaseError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Connection::open(&database_url)?;
    Ok(conn)
}

pub fn setup_database() -> Result<(), DatabaseError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_path = Path::new(&database_url);

    if let Some(parent) = database_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS aboutme (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            active BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}
pub fn create_contact_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contact (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            admin_mail TEXT NOT NULL,
            active BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

pub fn create_users_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
pub fn insert_user_from_env() -> Result<(), DatabaseError> {
    let conn = get_connection()?;

    let username = std::env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME must be set");
    let password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        &[&username, &password],
    )?;

    Ok(())
}

pub fn create_goals_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS goals (
            id INTEGER PRIMARY KEY,
            campaign_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            amount REAL NOT NULL,
            FOREIGN KEY(campaign_id) REFERENCES campaigns(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_campaigns_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS campaigns (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            creator_id INTEGER NOT NULL,
            start_date TEXT NOT NULL,
            end_date TEXT NOT NULL,
            goal_amount REAL NOT NULL,
            current_amount REAL NOT NULL DEFAULT 0,
            status TEXT NOT NULL,
            xmr_address TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_campaign(
    title: &str,
    description: &str,
    creator_id: i64,
    start_date: &str,
    end_date: &str,
    goal_amount: f64,
    status: &str,
    xmr_address: &str,
) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO campaigns (title, description, creator_id, start_date, end_date, goal_amount, status, xmr_address) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![title, description, creator_id, start_date, end_date, goal_amount, status, xmr_address],
    )?;
    Ok(())
}

pub fn get_campaigns() -> Result<Vec<(i64, String, String, f64, f64, String, String, String, String)>, DatabaseError> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, description, goal_amount, current_amount, status, xmr_address, start_date, end_date FROM campaigns",
    )?;
    let campaign_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
        ))
    })?;

    let mut campaigns = Vec::new();
    for campaign in campaign_iter {
        campaigns.push(campaign?);
    }
    Ok(campaigns)
}



pub fn update_campaign_current_amount(campaign_id: i64, amount: f64) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE campaigns SET current_amount = current_amount + ? WHERE id = ?",
        params![amount, campaign_id],
    )?;
    Ok(())
}

pub fn delete_campaign(campaign_id: i64) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute("DELETE FROM campaigns WHERE id = ?", params![campaign_id])?;
    Ok(())
}
pub fn create_donation_crypto_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS donation_crypto (
            id INTEGER PRIMARY KEY,
            campaign_id INTEGER NOT NULL,
            cryptoname TEXT NOT NULL,
            wallet_address TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_donation_crypto(cryptoname: &str, wallet_address: &str) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO donation_crypto (cryptoname, wallet_address) VALUES (?, ?)",
        params![cryptoname, wallet_address],
    )?;
    Ok(())
}
pub fn get_all_donation_cryptos() -> Result<Vec<(i64, String, String)>, DatabaseError> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, cryptoname, wallet_address FROM donation_crypto")?;
    let crypto_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    let mut cryptos = Vec::new();
    for crypto in crypto_iter {
        cryptos.push(crypto?);
    }
    Ok(cryptos)
}
pub fn update_donation_crypto(
    id: i64,
    cryptoname: &str,
    wallet_address: &str,
) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE donation_crypto SET cryptoname = ?, wallet_address = ? WHERE id = ?",
        params![cryptoname, wallet_address, id],
    )?;
    Ok(())
}
pub fn delete_donation_crypto(id: i64) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute("DELETE FROM donation_crypto WHERE id = ?", params![id])?;
    Ok(())
}
pub fn update_campaign(
    id: i64,
    title: String,
    description: String,
    start_date: String,
    end_date: String,
    goal_amount: f64,
    xmr_address: String,
) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE campaigns SET 
            title = ?, 
            description = ?, 
            start_date = ?,
            end_date = ?,
            goal_amount = ?,
            xmr_address = ?
        WHERE id = ?",
        params![title, description, start_date, end_date, goal_amount, xmr_address, id],
    )?;
    Ok(())
}

pub fn create_campaign_updates_table() -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS campaign_updates (
            id INTEGER PRIMARY KEY,
            campaign_id INTEGER NOT NULL,
            update_text TEXT NOT NULL,
            update_hash TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(campaign_id) REFERENCES campaigns(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn add_campaign_update(campaign_id: i64, update_text: &str) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    let update_hash = format!("{:x}", md5::compute(update_text));
    
    conn.execute(
        "INSERT INTO campaign_updates (campaign_id, update_text, update_hash) VALUES (?, ?, ?)",
        params![campaign_id, update_text, update_hash],
    )?;
    Ok(())
}

pub fn get_campaign_updates(campaign_id: i64) -> Result<Vec<(i64, String, String, String)>, DatabaseError> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, update_text, update_hash, created_at FROM campaign_updates WHERE campaign_id = ? ORDER BY created_at DESC"
    )?;
    
    let updates = stmt.query_map([campaign_id], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;

    let mut result = Vec::new();
    for update in updates {
        result.push(update?);
    }
    Ok(result)
}
pub fn update_campaign_amount(id: i64, new_amount: f64) -> Result<(), DatabaseError> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE campaigns SET current_amount = ? WHERE id = ?",
        params![new_amount, id],
    )?;
    Ok(())
}
pub fn get_campaign_amount(id: i64) -> Result<f64, DatabaseError> {
    let conn = get_connection()?;
    let current_amount: f64 = conn.query_row(
        "SELECT current_amount FROM campaigns WHERE id = ?",
        [id],
        |row| row.get(0)
    )?;
    Ok(current_amount)
}
