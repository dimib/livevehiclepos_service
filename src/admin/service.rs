use rocket::serde::json::{Value, json};
use rocket::http::Status;

use crate::simulation::simulation::reload_simulation;

#[get("/reload", format = "json")]
pub async fn reload() -> (Status, Value) {
    // Reload the simulation
    reload_simulation();

    (Status::Ok, json!({ "message": "Simulation reloaded" }))
}