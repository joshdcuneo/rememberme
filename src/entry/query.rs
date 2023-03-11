use super::{CreateEntry, UpdateEntry};
use crate::entry::Entry;
use crate::error;
use sqlx::SqlitePool;

pub async fn list_all(pool: &SqlitePool) -> error::Result<Vec<Entry>> {
    let entries = sqlx::query_as!(Entry, "SELECT name, description, slug FROM entries")
        .fetch_all(pool)
        .await?;

    Ok(entries)
}

pub async fn get_one_by_slug(pool: &SqlitePool, slug: &str) -> error::Result<Entry> {
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        slug
    )
    .fetch_one(pool)
    .await?;

    Ok(entry)
}

pub async fn create_one(pool: &SqlitePool, entry: &CreateEntry) -> error::Result<Entry> {
    sqlx::query!(
        "INSERT INTO entries (name, description, slug) VALUES (?, ?, ?)",
        entry.name,
        entry.description,
        entry.slug
    )
    .execute(pool)
    .await?;

    get_one_by_slug(pool, &entry.slug).await
}

pub async fn update_one(
    pool: &SqlitePool,
    slug: &str,
    entry: &UpdateEntry,
) -> error::Result<Entry> {
    sqlx::query!(
        "UPDATE entries SET name = ?, description = ? WHERE slug = ?",
        entry.name,
        entry.description,
        slug
    )
    .execute(pool)
    .await?;

    get_one_by_slug(pool, slug).await
}

pub async fn delete_one(pool: &SqlitePool, slug: &str) -> error::Result<Entry> {
    let entry = get_one_by_slug(pool, slug).await?;

    sqlx::query!("DELETE FROM entries WHERE slug = ?", slug)
        .execute(pool)
        .await?;

    Ok(entry)
}
