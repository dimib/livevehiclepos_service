use rocket::serde::json::{Value, json};
use rocket::http::Status;
use rocket::Request;

use crate::simulation::simulation::get_next_position;
use crate::positions::model::Position;

#[get("/?<vehicleIds>", format = "json")]
pub async fn get_positions(vehicleIds: String) -> (Status, Value) {

    let vehicle_ids = vehicleIds.split(",").collect::<Vec<&str>>();
    let mut positions = Vec::<Position>::new();
    
    for id in vehicle_ids {
        match get_next_position(&id.to_string()) {
            Some(position) => positions.push(position),
            None => (),
        }
    }

    (Status::Ok, json!(positions))
}

#[catch(401)]
pub fn not_unauthorized(_req: &Request) -> String {
    format!("Not authorized")
}

#[catch(404)]
pub fn not_found(_req: &Request) -> String {
    format!("Not found")
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> String {
    format!("Unprocessable entity")
}