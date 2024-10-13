use rocket::{delete, get, post, put};
use rocket::response::status;
use rocket::serde::json::json;
use serde_json::Value;
use crate::basic_auth::BasicAuth;

#[get("/rustaceans")]
pub fn get_rustaceans(auth: BasicAuth) -> Value {
    println!("{:?}", auth);
    json!([
        { "id": 1, "name": "John Doe", "age": 30 },
        { "id": 1, "name": "Jane Doe", "age": 28 },
    ])
}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[post("/rustaceans", format = "json")]
pub fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({ "id": 3, "name": "John Doe", "age": 30 })
}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: u32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "John Doe", "age": 30 })
}

#[delete("/rustaceans/<_id>")]
pub fn delete_rustacean(_id: u32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}