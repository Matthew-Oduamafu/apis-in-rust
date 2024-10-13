#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{Value, json};

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([
        { "id": 1, "name": "John Doe", "age": 30 },
        { "id": 1, "name": "Jane Doe", "age": 28 },
    ])
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({ "id": 3, "name": "John Doe", "age": 30 })
}

#[put("/rustaceans/<id>")]
fn update_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: u32) -> status::NoContent {
    status::NoContent
}

// handle 404
#[catch(404)]
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found"})
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/api/", routes![get_rustaceans, view_rustacean, create_rustacean, update_rustacean, delete_rustacean])
        .register("/", catchers![not_found])
        .launch()
        .await;
}