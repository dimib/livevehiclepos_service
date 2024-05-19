use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub lastReceived: String,
    pub position: PositionData,
    pub vehicleId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionData {
    pub latitude: f64,
    pub longitude: f64,
}
