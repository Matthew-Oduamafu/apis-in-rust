use rocket::{delete, get, post, put};
use rocket::response::status;
use rocket::serde::json::json;
use serde_json::Value;

#[get("/rustaceans")]
pub fn get_rustaceans() -> Value {
    json!([
        { "id": 1, "name": "John Doe", "age": 30 },
        { "id": 1, "name": "Jane Doe", "age": 28 },
    ])
}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[post("/rustaceans", format = "json")]
pub fn create_rustacean() -> Value {
    json!({ "id": 3, "name": "John Doe", "age": 30 })
}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: u32) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[delete("/rustaceans/<_id>")]
pub fn delete_rustacean(_id: u32) -> status::NoContent {
    status::NoContent
}