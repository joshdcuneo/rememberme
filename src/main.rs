use actix_web::{error, App, Error, HttpServer};
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, Json},
    Apiv2Schema, OpenApiExt,
};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use env_logger::Env;

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Entry {
    name: String,
    description: String,
    slug: String,
}

#[api_v2_operation]
#[get("/entries")]
async fn list_entries(pool: web::Data<Pool<Sqlite>>) -> Result<Json<Vec<Entry>>, Error> {
    let entries = sqlx::query_as!(Entry, "SELECT name, description, slug FROM entries")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Error: {}", e);
            error::ErrorInternalServerError("Database error".to_string())
        })?;

    Ok(Json(entries))
}

#[api_v2_operation]
#[get("/entries/{slug}")]
async fn show_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let slug = slug.into_inner();
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        slug
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[post("/entries")]
async fn create_entry(
    body: Json<Entry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    sqlx::query!(
        "INSERT INTO entries (name, description, slug) VALUES (?, ?, ?)",
        body.name,
        body.description,
        body.slug
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        body.slug
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[put("/entries/{slug}")]
async fn update_entry(
    slug: web::Path<String>,
    body: Json<Entry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let slug = slug.to_string();
    sqlx::query!(
        "UPDATE entries SET name = ?, description = ? WHERE slug = ?",
        body.name,
        body.description,
        slug
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        body.slug
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[delete("/entries/{slug}")]
async fn delete_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let slug = slug.to_string();
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        slug
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Error: {}", e);
        error::ErrorInternalServerError("Database error".to_string())
    })?;
    sqlx::query!("DELETE FROM entries WHERE slug = ?", slug)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("Error: {}", e);
            error::ErrorInternalServerError("Database error".to_string())
        })?;

    Ok(Json(entry))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let pool = SqlitePoolOptions::new().connect("sqlite.db").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap_api()
            .service(list_entries)
            .service(show_entry)
            .service(create_entry)
            .service(update_entry)
            .service(delete_entry)
            .with_json_spec_at("/openapi.json")
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
