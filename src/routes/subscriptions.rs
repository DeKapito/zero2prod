use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<SqlitePool>) -> impl Responder {
    let uuid = Uuid::new_v4();
    let current_time = Utc::now();
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid,
        form.email,
        form.name,
        current_time,
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError()
        }
    }
}
