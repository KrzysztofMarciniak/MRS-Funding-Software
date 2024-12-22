use crate::{db::get_connection, render_page_or_error};
use crate::views::layout::render_layout;
use axum::{
    extract::{Form, Path},
    response::{Html, Redirect},
};
use axum_csrf::CsrfToken;
use hyper::StatusCode;
use rusqlite::params;
use serde::Deserialize;
use crate::controllers::page::Page;

#[derive(Deserialize)]
pub struct ContactForm {
    description: String,
    admin_mail: String,
}

pub async fn contact(session: &tower_sessions::Session) -> Html<String> {
    let mut page = Page::new("Contact Us", session)
        .with_meta_description("Get in touch with our team");

    let conn = get_connection().unwrap();
    let result = conn
        .query_row(
            "SELECT description, admin_mail FROM contact WHERE active = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or_else(|_| {
            (
                "Contact information will be available soon".to_string(),
                "admin@example.com".to_string(),
            )
        });

    let content = format!(
        r#"
        <section>
            <h2>Contact Us 📬</h2>
            <div class="contact-info">{}</div>
            <div class="email-info">Email: {}</div>
        </section>
        "#,
        result.0, result.1
    );

    page.set_content(content);
    render_page_or_error!(page, "contact page")
}

pub async fn contact_create(
    csrf_token: CsrfToken,
    session: &tower_sessions::Session,
) -> Html<String> {
    let mut page = Page::new("Create Contact Information", session)
        .with_csrf_token(csrf_token)
        .with_meta_description("Create new contact information");

    let content = format!(
        r#"
        <section>
            <h2>Create New Contact Information</h2>
            <h3>You can use HTML in the description.</h3>
            <form method="POST" action="/protected/contact/new">
                <input type="hidden" name="csrf_token" value="{}">
                <div>
                    <label>Description:</label>
                    <textarea name="description" required></textarea>
                </div>
                <div>
                    <label>Admin Email:</label>
                    <input type="email" name="admin_mail" required>
                </div>
                <button type="submit">Create</button>
            </form>
        </section>
        "#,
        page.get_csrf_token().unwrap_or(&String::new())
    );

    page.set_content(content);
    page.render().await.unwrap_or_else(|_| Html(String::from("Error creating contact page")))
}


pub async fn contact_insert_created(Form(form): Form<ContactForm>) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();
    conn.execute(
        "INSERT INTO contact (description, admin_mail, active) VALUES (?, ?, 0)",
        params![form.description, form.admin_mail],
    )
    .unwrap();

    Ok(Redirect::to("/protected/contact/all"))
}

#[derive(Deserialize)]
pub struct ContactUpdateForm {
    description: String,
    admin_mail: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_checkbox")]
    active: bool,
}

fn get_all_contact_records() -> Vec<(i64, String, String, bool)> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, description, admin_mail, active FROM contact ORDER BY id DESC")
        .unwrap();

    stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })
    .unwrap()
    .map(|r| r.unwrap())
    .collect()
}

pub async fn contact_all(session: &tower_sessions::Session) -> Result<Html<String>, StatusCode> {
    let contact_list = get_all_contact_records();

    let content = format!(
        r#"
        <section>
            <h2>All Contact Entries</h2>
            <div class="contact-entries">
                {}
            </div>
            <a href="/protected/contact/new" class="button">Create New Contact</a>
        </section>
        "#,
        contact_list.iter()
            .map(|(id, desc, email, active)| format!(
                "<div class='entry'><p><a href='/protected/contact/{}'>{}</a></p><p>{}</p><p>Email: {}</p><p>Status: {}</p></div>",
                id, id, desc, email, if *active { "Active" } else { "Inactive" }
            ))
            .collect::<String>()
    );

    let rendered = render_layout(&content, session).await;
    Ok(rendered)
}
fn select_contact_by_id(id: i64) -> (i64, String, String, bool) {
    let conn = get_connection().unwrap();
    conn.query_row(
        "SELECT id, description, admin_mail, active FROM contact WHERE id = ?",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )
    .unwrap()
}

pub async fn contact_details(
    Path(id): Path<i64>,
    csrf_token: CsrfToken,
    session: &tower_sessions::Session,
) -> Result<Html<String>, StatusCode> {
    let contact = select_contact_by_id(id);
    let token_str = csrf_token.authenticity_token().unwrap();

    let content = format!(
        r#"
        <section>
            <h2>Contact Entry Details</h2>
            <h3>You can use HTML in the description.</h3>
            <div class="contact-entry">
                <form method="POST" action="/protected/contact/{}/update">
                    <input type="hidden" name="csrf_token" value="{}">
                    <div>
                        <label>Description:</label>
                        <textarea name="description" required>{}</textarea>
                    </div>
                    <div>
                        <label>Admin Email:</label>
                        <input type="email" name="admin_mail" value="{}" required>
                    </div>
                    <label>
                        <input type="checkbox" name="active" value="1" {}>
                        Active
                    </label>
                    <button type="submit">Update</button>
                </form>
                <form method="POST" action="/protected/contact/{}/delete" 
                    onsubmit="return confirm('Are you sure?')">
                    <input type="hidden" name="csrf_token" value="{}">
                    <button type="submit" class="delete">Delete</button>
                </form>
            </div>
        </section>
        "#,
        contact.0,
        token_str,
        contact.1,
        contact.2,
        if contact.3 { "checked" } else { "" },
        contact.0,
        token_str
    );
    let rendered = render_layout(&content, session).await;
    Ok(rendered)
}

pub async fn contact_update(
    Path(id): Path<i64>,
    Form(form): Form<ContactUpdateForm>,
) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();

    if form.active {
        conn.execute("UPDATE contact SET active = 0 WHERE active = 1", [])
            .unwrap();
    }

    conn.execute(
        "UPDATE contact SET description = ?, admin_mail = ?, active = ? WHERE id = ?",
        params![form.description, form.admin_mail, form.active, id],
    )
    .unwrap();

    Ok(Redirect::to("/protected/contact/all"))
}

pub async fn contact_delete(Path(id): Path<i64>) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();
    conn.execute("DELETE FROM contact WHERE id = ?", [id])
        .unwrap();
    Ok(Redirect::to("/protected/contact/all"))
}

fn deserialize_checkbox<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(string) => Ok(string == "1" || string == "on" || string == "true"),
        Err(_) => Ok(false),
    }
}
