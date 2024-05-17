#[macro_use] extern crate rocket;

mod positions;
mod simulation;

use simulation::simulation::setup_simulation;


#[launch]
fn rocket() -> _ {

    setup_simulation();

    rocket::build()
        .mount("/positions", routes![
            positions::service::get_positions
        ])
        .register("/", catchers![
            positions::service::not_unauthorized,
            positions::service::not_found,
            positions::service::unprocessable_entity
        ])
}