#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};
use rocket_app::rustaceans_api::{create_rustacean, delete_rustacean, get_rustaceans, update_rustacean, view_rustacean};

// handle 404
#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found"})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/api/", routes![get_rustaceans, view_rustacean, create_rustacean, update_rustacean, delete_rustacean])
        .register("/", catchers![not_found])
        .ignite().await?
        .launch().await?;

    Ok(())
}