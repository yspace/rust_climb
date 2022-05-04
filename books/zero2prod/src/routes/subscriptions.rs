use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use sqlx::PgConnection ;

use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(
    form: web::Form<FormData>
    ,connection: web::Data<PgConnection>
) -> HttpResponse {
    // sqlx::query!(
    //     r#"
    //     INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
    //     "#,
    //     Uuid::new_v4(),
    //     form.email,
    //     form.name,
    //     Utc::now()
    //     )
    //     // We use `get_ref` to get an immutable reference to the `PgConnection` // wrapped by `web::Data`.
    //     .execute(connection.get_ref())
    //     .await;
    HttpResponse::Ok().finish()
}