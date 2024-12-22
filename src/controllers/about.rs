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
pub struct AboutForm {
    description: String,
}

pub async fn about(session: &tower_sessions::Session) -> Html<String> {
    let mut page = Page::new("About Us 👥", session)
        .with_meta_description("Learn more about our crowdfunding platform");

    let conn = get_connection().unwrap();
    let description = conn
        .query_row(
            "SELECT description FROM aboutme WHERE active = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "Please configure about me in dashboard".to_string());

    let content = format!(
        r#"
        <section>
            <h2>About Us</h2>
            <div class="about-content">{}</div>
        </section>
        "#,
        description
    );

    page.set_content(content);
    page.render().await.unwrap_or_else(|_| Html(String::from("Error loading about page")))
}

pub async fn about_create(
    csrf_token: CsrfToken,
    session: &tower_sessions::Session,
) -> Html<String> {
    let mut page = Page::new("Create New About Entry", session)
        .with_csrf_token(csrf_token)
        .with_meta_description("Create a new about page entry");

    let content = format!(
        r#"
        <section>
            <h2>Create New About Entry</h2>
            <h3>You can use HTML in the description.</h3>
            <form method="POST" action="/protected/about/new">
                <input type="hidden" name="csrf_token" value="{}">
                <textarea name="description" required></textarea>
                <button type="submit">Create</button>
            </form>
        </section>
        "#,        
        page.get_csrf_token().unwrap_or(&String::new())
    );

    page.set_content(content);
    render_page_or_error!(page, "about page")
}
pub async fn about_insert_created(Form(form): Form<AboutForm>) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();

    conn.execute(
        "INSERT INTO aboutme (description, active) VALUES (?, 0)",
        [&form.description],
    )
    .unwrap();

    Ok(Redirect::to("/protected/about/all"))
}

fn get_all_about_records() -> Vec<(i64, String, bool)> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, description, active FROM aboutme ORDER BY id DESC")
        .unwrap();

    stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
}

pub async fn about_all(session: &tower_sessions::Session) -> Result<Html<String>, StatusCode> {
    let about_list = get_all_about_records();

    let content = format!(
        r#"
        <section>
            <h2>All About Entries</h2>
            <div class="about-entries">
                {}
            </div>
            <a href="/protected/about/new" class="button">Create New Entry</a>
        </section>
        "#,
        about_list.iter()
            .map(|(id, desc, active)| format!(
                "<div class='entry'><p><a href='/protected/about/{}'>{}</a></p><p>{}</p><p>Status: {}</p></div>",
                id, id, desc, if *active { "Active" } else { "Inactive" }
            ))
            .collect::<String>()
    );

    let rendered = render_layout(&content, session).await;
    Ok(rendered)
}
fn select_from_id(id: i64) -> (i64, String, bool) {
    let conn = get_connection().unwrap();
    conn.query_row(
        "SELECT id, description, active FROM aboutme WHERE id = ?",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )
    .unwrap()
}

pub async fn about_details(
    Path(id): Path<i64>,
    csrf_token: CsrfToken,
    session: &tower_sessions::Session,
) -> Result<Html<String>, StatusCode> {
    let about = select_from_id(id);
    let token_str = csrf_token.authenticity_token().unwrap();

    let content = format!(
        r#"
        <section>
            <h2>About Entry Details</h2>
            <h3>You can use HTML in the description.</h3>
            <div class="about-entry">
                <form method="POST" action="/protected/about/{}/update">
                    <input type="hidden" name="csrf_token" value="{}">
                    <textarea name="description" required>{}</textarea>
                    <label>
                        <input type="checkbox" name="active" value="1" {}>
                        Active
                    </label>
                    <button type="submit">Update</button>
                </form>
                <form method="POST" action="/protected/about/{}/delete" 
                    onsubmit="return confirm('Are you sure?')">
                    <input type="hidden" name="csrf_token" value="{}">
                    <button type="submit" class="delete">Delete</button>
                </form>
            </div>
        </section>
        "#,
        about.0,
        token_str,
        about.1,
        if about.2 { "checked" } else { "" },
        about.0,
        token_str
    );
    let rendered = render_layout(&content, session).await;
    Ok(rendered)
}
pub async fn about_update(
    Path(id): Path<i64>,
    Form(form): Form<AboutUpdateForm>,
) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();

    if form.active {
        conn.execute("UPDATE aboutme SET active = 0 WHERE active = 1", [])
            .unwrap();
    }

    conn.execute(
        "UPDATE aboutme SET description = ?, active = ? WHERE id = ?",
        params![form.description, form.active, id],
    )
    .unwrap();

    Ok(Redirect::to("/protected/about/all"))
}

pub async fn about_delete(Path(id): Path<i64>) -> Result<Redirect, StatusCode> {
    let conn = get_connection().unwrap();
    conn.execute("DELETE FROM aboutme WHERE id = ?", [id])
        .unwrap();
    Ok(Redirect::to("/protected/about/all"))
}
#[derive(Deserialize)]
pub struct AboutUpdateForm {
    description: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_checkbox")]
    active: bool,
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
