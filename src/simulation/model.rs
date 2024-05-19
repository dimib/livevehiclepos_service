use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePositionFile {
    pub coords: Vec<VehiclePosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehiclePosition {
    pub time: String,
    pub lat: f64, 
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleSimualationFile {
    pub vehicle_simulations: Vec<VehicleSimulation>,
    pub vehicle_id_mapping: Vec<RealVehicleToSimVehicle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleSimulation {
    pub vehicle_id: String,
    pub pos_file: String,
    pub step: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealVehicleToSimVehicle {
    pub real_vehicle_ids: String,
    pub sim_vehicle_id: String,
}


