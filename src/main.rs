mod entry;
mod error;

use actix_web::middleware::Logger;
use actix_web::{App, Error, HttpServer};
use entry::{CreateEntry, Entry, UpdateEntry};
use env_logger::Env;
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, Json},
    OpenApiExt,
};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[api_v2_operation]
#[get("/entries")]
async fn list_entries(pool: web::Data<Pool<Sqlite>>) -> Result<Json<Vec<Entry>>, Error> {
    let entries = entry::query::list_all(pool.get_ref()).await?;

    Ok(Json(entries))
}

#[api_v2_operation]
#[get("/entries/{slug}")]
async fn show_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::get_one_by_slug(pool.get_ref(), &slug.into_inner()).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[post("/entries")]
async fn create_entry(
    body: Json<CreateEntry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::create_one(pool.get_ref(), &body).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[put("/entries/{slug}")]
async fn update_entry(
    slug: web::Path<String>,
    body: Json<UpdateEntry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::update_one(pool.get_ref(), &slug.into_inner(), &body).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[delete("/entries/{slug}")]
async fn delete_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::delete_one(pool.get_ref(), &slug.into_inner()).await?;

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
