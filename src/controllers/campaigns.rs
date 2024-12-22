use crate::render_page_or_error;
use crate::{controllers::page::Page, db};
use crate::views::layout::render_layout;
use axum::{
    extract::{Form, Path},
    http::StatusCode,
    response::{Html, Redirect},
};
use axum_csrf::CsrfToken;
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct CampaignForm {
    title: String,
    description: String,
    start_date: String,
    end_date: String,
    goal_amount: f64,
    xmr_address: String,
}
pub async fn new_campaign_page(csrf_token: CsrfToken, session: &Session) -> Html<String> {
    let token_str = csrf_token.authenticity_token().unwrap();

    let content = format!(
        r#"
        <section class="campaign-form">
            <form method="POST" action="/protected/campaigns/new">
                <input type="hidden" name="csrf_token" value="{}">
                <div class="form-group">
                    <label for="title">Title:</label>
                    <input type="text" name="title" required>
                </div>
                <div class="form-group">
                    <label for="description">Description:</label>
                    <textarea name="description" required></textarea>
                </div>
                <div class="form-group">
                    <label for="start_date">Start Date:</label>
                    <input type="date" name="start_date" required>
                </div>
                <div class="form-group">
                    <label for="end_date">End Date:</label>
                    <input type="date" name="end_date" required>
                </div>
                <div class="form-group">
                    <label for="goal_amount">Goal Amount:</label>
                    <input type="number" step="0.01" name="goal_amount" required>
                </div>
                    <div class="form-group">
                    <label for="xmr_address">Monero Address:</label>
                    <input type="text" name="xmr_address" required>
                </div>
                <button type="submit">Create Campaign</button>
            </form>
        </section>
        "#,
        token_str
    );
    render_layout(&content, session).await
}

pub async fn create_campaign(
    session: Session,
    Form(form): Form<CampaignForm>,
) -> Result<Redirect, (StatusCode, String)> {
    let creator_id: i64 = match session.get::<String>("user_id").await {
        Ok(Some(user_id)) => user_id.parse().unwrap_or(1),
        _ => 0,
    };

    db::insert_campaign(
        &form.title,
        &form.description,
        creator_id,
        &form.start_date,
        &form.end_date,
        form.goal_amount,
        "active",
        &form.xmr_address,
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/"))
}

pub async fn list_campaigns(session: &Session) -> Html<String> {
    let campaigns = db::get_campaigns().unwrap_or_else(|_| vec![]);

    let mut content = String::from("<section class=\"campaign-list\">");
    for (id, title, description, goal_amount, current_amount, status, _xmr_address, start_date, end_date) in campaigns {
        content.push_str(&format!(
            r#"
            <div class="campaign">
                <h2>{}</h2>
                <p>{}</p>
                <p>Goal: {} XMR</p>
                <p>Current: {} XMR</p>
                <p>Status: {}</p>
                <p>Start Date: {}</p>
                <p>End Date: {}</p>
                <a href="/campaigns/{}">Details</a>
            </div>
            "#,
            title, description, goal_amount, current_amount, status, start_date, end_date, id
        ));
    }
    content.push_str("</section>");
    render_layout(&content, session).await
}

pub async fn campaign_details(Path(id): Path<i64>, session: &Session) -> Html<String> {
    let campaigns = db::get_campaigns().unwrap_or_else(|_| vec![]);
    let updates = db::get_campaign_updates(id).unwrap_or_else(|_| vec![]);

    let campaign = campaigns
        .into_iter()
        .find(|(cid, _, _, _, _, _, _, _, _)| *cid == id);

    let mut content = if let Some((_, title, description, goal_amount, current_amount, status, xmr_address, start_date, end_date)) = campaign {
        format!(
            r#"
            <section class="campaign-details">
                <h1>{}</h1>
                <p>{}</p>
                <p>Goal Amount: {} XMR</p>
                <p>Current Amount: {} XMR</p>
                <p>Status: {}</p>
                <p>XMR Address: {}</p>
                <p>Start Date: {}</p>
                <p>End Date: {}</p>
                
                <div class="campaign-updates">
                    <h2>Campaign Updates</h2>
                    "#,
            title, description, goal_amount, current_amount, status, xmr_address, start_date, end_date
        )
    } else {
        return render_layout("Campaign not found", session).await;
    };

    for (_, update_text, update_hash, created_at) in updates {
        content.push_str(&format!(
            r#"
            <div class="update-entry">
                <p>{}</p>
                <small>Posted: {} (Hash: {})</small>
            </div>
            "#,
            update_text, created_at, update_hash
        ));
    }

    content.push_str("</div></section>");
    render_layout(&content, session).await
}pub async fn update_campaign(
    Path(id): Path<i64>,
    Form(form): Form<CampaignForm>,
) -> Result<Redirect, (StatusCode, String)> {
    db::update_campaign_current_amount(id, form.goal_amount)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Redirect::to(&format!("/campaigns/{}", id)))
}

pub async fn delete_campaign(Path(id): Path<i64>) -> Result<Redirect, (StatusCode, String)> {
    db::delete_campaign(id).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Redirect::to("/protected/dashboard"))
}
pub async fn list_all_campaigns(session: &Session) -> Html<String> {
    let mut page = Page::new("Campaign Management", session)
        .with_meta_description("Manage all crowdfunding campaigns");

    let campaigns = db::get_campaigns().unwrap_or_else(|_| vec![]);
    let mut content = String::from(r#"<section class="campaigns-admin">
        <h2>Campaign Management</h2>
        <a href="/protected/campaigns/new" class="button">Create New Campaign</a>
        <div class="campaigns-list">"#);

    for (id, title, description, goal_amount, current_amount, status, xmr_address, start_date, end_date) in campaigns {
        content.push_str(&format!(
            r#"<div class="campaign-entry">
                <h3>{}</h3>
                <p>{}</p>
                <div class="campaign-stats">
                    <span>Goal: {} XMR</span>
                    <span>Current: {} XMR</span>
                    <span>Status: {}</span>
                    <span>Start Date: {}</span>
                    <span>End Date: {}</span>
                </div>
                <div class="campaign-actions">
                    <a href="/protected/campaigns/{}/edit" class="button">Edit</a>
                    <a href="/protected/campaigns/{}/amount" class="button">Update Amount</a>
                    <form method="POST" action="/protected/campaigns/{}/delete" 
                        onsubmit="return confirm('Are you sure you want to delete this campaign?')">
                        <button type="submit" class="button delete">Delete</button>
                    </form>
                </div>
            </div>"#,
            title, description, goal_amount, current_amount, status, start_date, end_date, id, id, id
        ));
    }
    content.push_str("</div></section>");
    
    page.set_content(content);
    render_page_or_error!(page, "Campaign Management")
}pub async fn edit_campaign_page(
    Path(id): Path<i64>,
    csrf_token: CsrfToken,
    session: &Session
) -> Html<String> {
    let mut page = Page::new("Edit Campaign", session)
        .with_csrf_token(csrf_token)
        .with_meta_description("Edit crowdfunding campaign details");

    let campaigns = db::get_campaigns().unwrap_or_else(|_| vec![]);
    let campaign = campaigns.into_iter().find(|(cid, _, _, _, _, _, _, _, _)| *cid == id);

    if let Some((_, title, description, goal_amount, current_amount, status, xmr_address, start_date, end_date)) = campaign {
        let content = format!(
            r#"<section class="campaign-form">
                <h2>Edit Campaign</h2>
                <form method="POST" action="/protected/campaigns/{}/edit">
                    <input type="hidden" name="csrf_token" value="{}">
                    <div class="form-group">
                        <label for="title">Title:</label>
                        <input type="text" name="title" value="{}" required>
                    </div>
                    <div class="form-group">
                        <label for="description">Description:</label>
                        <textarea name="description" required>{}</textarea>
                    </div>
                    <div class="form-group">
                        <label for="start_date">Start Date:</label>
                        <input type="date" name="start_date" value="{}" required>
                    </div>
                    <div class="form-group">
                        <label for="end_date">End Date:</label>
                        <input type="date" name="end_date" value="{}" required>
                    </div>
                    <div class="form-group">
                        <label for="goal_amount">Goal Amount:</label>
                        <input type="number" step="0.01" name="goal_amount" value="{}" required>
                    </div>
                    <div class="form-group">
                        <label for="xmr_address">Monero Address:</label>
                        <input type="text" name="xmr_address" value="{}" required>
                    </div>
                    <button type="submit">Update Campaign</button>
                </form>
            </section>"#,
            id,
            page.get_csrf_token().unwrap_or(&String::new()),
            title, description, start_date, end_date, goal_amount, xmr_address
        );
        page.set_content(content);
    }
    

    render_page_or_error!(page, "Edit Campaign")
}


#[derive(Deserialize)]
pub struct EditCampaignForm {
    title: String,
    description: String,
    start_date: String,
    end_date: String,
    goal_amount: f64,
    xmr_address: String,
}

pub async fn edit_campaign(
    Path(id): Path<i64>,
    Form(form): Form<CampaignForm>,
) -> Result<Redirect, (StatusCode, String)> {
    db::update_campaign(
        id,
        form.title,
        form.description,
        form.start_date,
        form.end_date, 
        form.goal_amount,
        form.xmr_address
    )
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Redirect::to("/protected/campaigns"))
}
#[derive(Deserialize)]
pub struct AmountUpdateForm {
    current_amount: f64,
}

pub async fn update_amount_page(
    Path(id): Path<i64>,
    csrf_token: CsrfToken,
    session: &Session
) -> Html<String> {
    let mut page = Page::new("Update Amount", session)
        .with_csrf_token(csrf_token);

    let current_amount = db::get_campaign_amount(id)
        .unwrap_or(0.0);

    let content = format!(
        r#"<section class="amount-form">
            <h2>Update Current Amount</h2>
            <p class="current-amount-display">Current Amount: {} XMR</p>
            <form method="POST" action="/protected/campaigns/{}/amount">
                <input type="hidden" name="csrf_token" value="{}">
                <div class="form-group">
                    <label for="current_amount">New Amount:</label>
                    <input type="number" step="0.01" name="current_amount" value="{}" required>
                </div>
                <button type="submit">Update Amount</button>
            </form>
        </section>"#,
        current_amount,
        id,
        page.get_csrf_token().unwrap_or(&String::new()),
        current_amount
    );
    page.set_content(content);
    render_page_or_error!(page, "Update Amount")
}


pub async fn update_campaign_amount(
    Path(id): Path<i64>,
    Form(form): Form<AmountUpdateForm>,
) -> Result<Redirect, (StatusCode, String)> {
    db::update_campaign_amount(id, form.current_amount)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Redirect::to("/protected/campaigns"))
}