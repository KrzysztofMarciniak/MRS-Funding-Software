use crate::controllers::page::Page;
use crate::{db, render_page_or_error};
use axum::response::Html;

pub async fn home(session: &tower_sessions::Session) -> Html<String> {
    let mut page = Page::new("Home", session)
        .with_meta_description("Decentralized Crowdfunding with Monero - Support innovative projects");

    let campaigns = db::get_campaigns().unwrap_or_else(|_| vec![]);

    let content = if campaigns.len() == 1 {
        let (id, title, description, goal_amount, current_amount, status, xmr_address, created_at, updated_at) = &campaigns[0];
        format!(
            r#"
            <section class="campaign-details">
                <h1>{} 🚀</h1>
                <p>{}</p>
                <div class="campaign-stats">
                    <p>Goal Amount: {} XMR 🎯</p>
                    <p>Current Amount: {} XMR 💰</p>
                    <p>Status: {} ✨</p>
                    <p>Monero Address: {} 🔒</p>
                    <p>Created: {}</p>
                    <p>Updated: {}</p>
                </div>
            </section>
            "#,
            title, description, goal_amount, current_amount, status, xmr_address, created_at, updated_at
        )
    } else {
        let mut content = String::from(r#"<section class="campaign-list"><h2>Active Campaigns 🎯</h2>"#);
        for (id, title, description, goal_amount, current_amount, status, xmr_address, created_at, updated_at) in campaigns {
            content.push_str(&format!(
                r#"
                <div class="campaign">
                    <h2>{} 🚀</h2>
                    <p>{}</p>
                    <div class="campaign-stats">
                        <p>Goal: {} XMR 🎯</p>
                        <p>Current: {} XMR 💰</p>
                        <p>Status: {} ✨</p>
                        <p>Monero Address: {} 🔒</p>
                        <p>Created: {}</p>
                        <p>Updated: {}</p>
                    </div>
                    <a href="/campaigns/{}" class="button">View Details →</a>
                </div>
                "#,
                title, description, goal_amount, current_amount, status, xmr_address, created_at, updated_at, id
            ));
        }
        content.push_str("</section>");
        content
    };

    page.set_content(content);
    render_page_or_error!(page, "home page")
}
