#[macro_use] extern crate rocket;

mod positions;
mod simulation;
mod admin;

use simulation::simulation::{setup_simulation, start_simulator_threads};

#[launch]
fn rocket() -> _ {

    setup_simulation();
    start_simulator_threads(1);

    rocket::build()
        .mount("/positions", routes![
            positions::service::get_positions
        ])
        .mount("/admin", routes![
            admin::service::reload
        ])
        .register("/", catchers![
            positions::service::not_unauthorized,
            positions::service::not_found,
            positions::service::unprocessable_entity
        ])
}