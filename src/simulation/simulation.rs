
use crate::simulation::model::vehicle_position_file;
use crate::positions::model::{Position, PositionData};

use std::fs::File;
use std::io::BufReader;

use chrono::DateTime;
use chrono::offset::Local;

static mut VEHICLE_IDS: Vec<String> = Vec::new();
static mut SIMULATION_FILES: Vec<vehicle_position_file> = Vec::new();
static mut CURRENT_INDEX: Vec<usize> = Vec::new();

pub fn setup_simulation() {
    unsafe {
        VEHICLE_IDS.push("23-2".to_string());
        SIMULATION_FILES.push(load_vehicle_positions("vehicle-23-2.pos.json".to_string()));
        CURRENT_INDEX.push(0);

        VEHICLE_IDS.push("16-1".to_string());
        SIMULATION_FILES.push(load_vehicle_positions("vehicle-16-1.pos.json".to_string()));
        CURRENT_INDEX.push(0);
    }
}

pub fn get_next_position(vehicle_id: &String) -> Option<Position> {
    unsafe {
        match index_of_vehicle_id(vehicle_id.to_string()) {
            Some(index) => {
                let position = &SIMULATION_FILES[index].coords[CURRENT_INDEX[index]];
                CURRENT_INDEX[index] += 1;
                if CURRENT_INDEX[index] >= SIMULATION_FILES[index].coords.len() {
                    CURRENT_INDEX[index] = 0;
                }
                let last_received = DateTime::<Local>::from(Local::now()).to_string();
                return Some(Position {
                    lastReceived: last_received.to_string(),
                    position: PositionData {
                        latitude: position.lat,
                        longitude: position.lon,
                    },
                    vehicleId: vehicle_id.to_string(),
                });
            }
            None => {
                return None;
            }
        }
    }
}

fn load_vehicle_positions(name: String) -> vehicle_position_file {
    // Load vehicle positions
    // ...
    println!("Loading vehicle positions");
    let file = File::open(name).expect("File not found");
    let reader = BufReader::new(file);
    let positions: vehicle_position_file = serde_json::from_reader(reader).expect("Error while reading file");
    return positions;
}

fn index_of_vehicle_id(vehicle_id: String) -> Option<usize> {
    unsafe {
        for i in 0..VEHICLE_IDS.len() {
            if VEHICLE_IDS[i] == vehicle_id {
                return Some(i);
            }
        }
    }
    return None;
}
