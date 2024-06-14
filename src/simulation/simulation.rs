
use crate::simulation::model::{VehiclePositionFile, VehicleSimualationFile, VehicleSimulation};
use crate::positions::model::{Position, PositionData};

use std::cmp::max;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::DateTime;
use chrono::offset::Local;
use chrono::SecondsFormat;

struct SimulationData {
    vehicle_id: String,
    simulation_file: VehiclePositionFile,
    current_index: usize,
    step: usize,
    current_time: DateTime<Local>,
}

static mut SIMULATION_DATA: Vec<SimulationData> = Vec::new();

static mut MUTEX: std::sync::Mutex<()> = Mutex::new(());

pub fn setup_simulation() {
    unsafe {
        let simulations = load_simulation_data();
        for simulation in simulations {
            let simulation_file = load_vehicle_positions(simulation.pos_file);
            SIMULATION_DATA.push(SimulationData {
                vehicle_id: simulation.vehicle_id,
                simulation_file: simulation_file,
                current_index: 0,
                step: simulation.step,
                current_time: DateTime::<Local>::from(Local::now()),
            });
        }
    }
}

pub fn start_simulator_threads(speed: u64) {

    println!("Starting simulation threads");

    let duration = 1 / max(speed, 1);

    thread::spawn(move || {
        loop {
            // Update simulation data
            // ...
            // println!("Updating simulation data");
            thread::sleep(Duration::from_secs(duration));
            unsafe {
                match MUTEX.lock() {
                    Ok(_) => {
                        for i in 0..SIMULATION_DATA.len() {
                            SIMULATION_DATA[i].current_index += SIMULATION_DATA[i].step;
                            if SIMULATION_DATA[i].current_index >= SIMULATION_DATA[i].simulation_file.coords.len() {
                                SIMULATION_DATA[i].current_index = 0;
                            }
                            SIMULATION_DATA[i].current_time = DateTime::<Local>::from(Local::now());
                        }        
                    }
                    Err(_) => {
                        println!("Mutex is locked");
                        continue;
                    }
                }
            }
        }
    });
}

pub fn reload_simulation() {
    unsafe {
        match MUTEX.lock() {
            Ok(_) => {
                SIMULATION_DATA.clear();
                setup_simulation();    
            }
            Err(_) => {
                println!("Mutex is locked");
            }
        }
    }
}

pub fn get_next_position(vehicle_id: &String) -> Option<Position> {
    unsafe {
        let sim_vehicle_id = get_sim_vehicle_id(vehicle_id.to_string());
        match MUTEX.lock() {
            Ok(_) => {
                match index_of_vehicle_id(sim_vehicle_id) {
                    Some(index) => {
                        let position = &SIMULATION_DATA[index].simulation_file.coords[SIMULATION_DATA[index].current_index];
                        let last_received = &SIMULATION_DATA[index].current_time.to_rfc3339_opts(SecondsFormat::Secs, true);

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
            Err(_) => {
                println!("Mutex is locked");
                return None;
            }
        }
    }
}

fn load_vehicle_positions(name: String) -> VehiclePositionFile {
    // Load vehicle positions
    // ...
    println!("Loading vehicle positions from file: {}", name);
    let file = File::open(name).expect("File not found");
    let reader = BufReader::new(file);
    let positions: VehiclePositionFile = serde_json::from_reader(reader).expect("Error while reading file");
    return positions;
}

fn index_of_vehicle_id(vehicle_id: String) -> Option<usize> {
    unsafe {
        for i in 0..SIMULATION_DATA.len() {
            if SIMULATION_DATA[i].vehicle_id == vehicle_id {
                return Some(i);
            }
        }
    }
    return None;
}

fn get_sim_vehicle_id(vehicle_id: String) -> String {
    let file = File::open("sim_vehicle.json").expect("File not found");
    let reader = BufReader::new(file);
    let mapping_file: VehicleSimualationFile = serde_json::from_reader(reader).expect("Error while reading file");

    for mapping in mapping_file.vehicle_id_mapping {
        if mapping.real_vehicle_ids.contains(&vehicle_id) {
            return mapping.sim_vehicle_id.to_string();
        }
    }

    return vehicle_id.to_string();
}

fn load_simulation_data() -> Vec<VehicleSimulation> {
    let file = File::open("sim_vehicle.json").expect("File not found");
    let reader = BufReader::new(file);
    let mapping_file: VehicleSimualationFile = serde_json::from_reader(reader).expect("Error while reading file");
    return mapping_file.vehicle_simulations;
}
