#[macro_use]
extern crate rocket;

mod db;
mod http;

use http::routes::*;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use sqlx::postgres::PgPool;
use std::env; // Import the missing index struct or variant

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool");

    // Call the function to create the users table
    db::crud::create_users_table(&db_pool)
        .await
        .expect("Failed to create users table");

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration failed");

    rocket::build().attach(cors).manage(db_pool).mount(
        "/",
        routes![
            index,
            register,
            login,
            admin_dashboard,
            teacher_dashboard,
            student_dashboard,
            ipm_dashboard
        ],
    )
}
