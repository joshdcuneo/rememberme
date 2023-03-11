use super::{CreateEntry, UpdateEntry};
use crate::entry::Entry;
use sqlx::SqlitePool;
use crate::error;

pub async fn list(pool: &SqlitePool) -> error::Result<Vec<Entry>> {
    let entries = sqlx::query_as!(Entry, "SELECT name, description, slug FROM entries")
        .fetch_all(pool)
        .await?;

    Ok(entries)
}

pub async fn get_optional_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> error::Result<Option<Entry>> {
    let entry = sqlx::query_as!(
        Entry,
        "SELECT name, description, slug FROM entries WHERE slug = ?",
        slug
    )
    .fetch_optional(pool)
    .await?;

    Ok(entry)
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

pub async fn create(pool: &SqlitePool, entry: &CreateEntry) -> error::Result<Entry> {
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

pub async fn update_optional(
    pool: &SqlitePool,
    slug: &str,
    entry: &UpdateEntry,
) -> error::Result<Option<Entry>> {
    let result = sqlx::query!(
        "UPDATE entries SET name = ?, description = ? WHERE slug = ?",
        entry.name,
        entry.description,
        slug
    )
    .execute(pool)
    .await;

    if result.is_ok() {
        return get_optional_by_slug(pool, slug).await;
    }

    Ok(None)
}

pub async fn delete_optional(
    pool: &SqlitePool,
    slug: &str,
) -> error::Result<Option<Entry>> {
    if let Some(entry) = get_optional_by_slug(pool, slug).await? {
        sqlx::query!("DELETE FROM entries WHERE slug = ?", slug)
            .execute(pool)
            .await?;

        return Ok(Some(entry));
    }

    Ok(None)
}
