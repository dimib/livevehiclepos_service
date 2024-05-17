use serde::{Serialize, Deserialize};

/**
 [
  {
    "lastReceived": "2024-05-17T05:24:40.799Z",
    "position": {
      "latitude": 9.98569,
      "longitude": 53.55017
    },
    "vehicleId": "string"
  }
]
 */

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
