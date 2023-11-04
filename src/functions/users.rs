// SQLX TYPES
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::Row;

// ENV
use crate::User;

use crate::db::connection::db_connection;

pub async fn get_users() -> Result<Vec<User>, sqlx::Error> {
    let mut connection = db_connection().await.unwrap();

    // Execute a SQL query.

    let select_query = sqlx::query("SELECT * FROM public.users");

    let results = select_query.fetch_all(&mut connection).await?;

    let mut users: Vec<User> = Vec::new();

    for result in results {
        let id: Uuid = result.try_get("id")?;
        let name: String = result.try_get("name")?;
        let created_at: DateTime<Utc> = result.try_get("created_at")?;
        let age: i32 = result.try_get("age")?;

        users.push({
            User {
                id,
                name,
                created_at,
                age,
            }
        })
    }

    Ok(users)
}
