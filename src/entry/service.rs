use crate::entry::{self, CreateEntry, Entry, UpdateEntry};
use actix_web::Error;
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, Json},
};
use sqlx::{Pool, Sqlite};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/entries")
            .service(list_entries)
            .service(show_entry)
            .service(create_entry)
            .service(update_entry)
            .service(delete_entry),
    );
}

#[api_v2_operation]
#[get()]
async fn list_entries(pool: web::Data<Pool<Sqlite>>) -> Result<Json<Vec<Entry>>, Error> {
    let entries = entry::query::list_all(pool.get_ref()).await?;

    Ok(Json(entries))
}

#[api_v2_operation]
#[get("/{slug}/")]
async fn show_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::get_one_by_slug(pool.get_ref(), &slug.into_inner()).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[post()]
async fn create_entry(
    body: Json<CreateEntry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::create_one(pool.get_ref(), &body).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[put("/{slug}/")]
async fn update_entry(
    slug: web::Path<String>,
    body: Json<UpdateEntry>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::update_one(pool.get_ref(), &slug.into_inner(), &body).await?;

    Ok(Json(entry))
}

#[api_v2_operation]
#[delete("/{slug}/")]
async fn delete_entry(
    slug: web::Path<String>,
    pool: web::Data<Pool<Sqlite>>,
) -> Result<Json<Entry>, Error> {
    let entry = entry::query::delete_one(pool.get_ref(), &slug.into_inner()).await?;

    Ok(Json(entry))
}
