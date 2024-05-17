use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct vehicle_position_file {
    pub coords: Vec<vehicle_position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct vehicle_position {
    pub time: String,
    pub lat: f64, 
    pub lon: f64,
}