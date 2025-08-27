use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub name: String,
    pub source: String,
    
    #[serde(rename = "vehicleType")]
    pub vehicle_type: Option<String>,
    
    pub size: Option<String>,
    pub page: Option<i32>,
    
    // Capacity
    #[serde(rename = "capCrew")]
    pub cap_crew: Option<i32>,
    #[serde(rename = "capPassenger")]
    pub cap_passenger: Option<i32>,
    #[serde(rename = "capCargo")]
    pub cap_cargo: Option<f32>,
    
    // Stats
    pub ac: Option<i32>,
    pub hp: Option<i32>,
    pub speed: Option<Speed>,
    pub pace: Option<i32>,
    
    // Dimensions [length, width]
    pub dimensions: Option<Vec<String>>,
    
    // Damage immunities, resistances, etc.
    pub immune: Option<Vec<String>>,
    pub resist: Option<Vec<String>>,
    pub vulnerable: Option<Vec<String>>,
    
    // Terrain types
    pub terrain: Option<Vec<String>>,
    
    // Weapons
    pub weapon: Option<Vec<VehicleWeapon>>,
    
    // Description entries
    pub entries: Option<Vec<serde_json::Value>>,
    
    // SRD name
    pub srd: Option<String>,
    
    // Fluff flags
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
    #[serde(rename = "hasToken")]
    pub has_token: Option<bool>,
    
    #[serde(flatten)]
    pub other_fields: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Speed {
    pub walk: Option<i32>,
    pub swim: Option<i32>,
    pub fly: Option<i32>,
    pub burrow: Option<i32>,
    pub climb: Option<i32>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleWeapon {
    pub name: String,
    pub count: Option<i32>,
    pub entries: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleData {
    pub vehicle: Option<Vec<Vehicle>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VehicleSummary {
    pub name: String,
    pub source: String,
    pub vehicle_type: Option<String>,
    pub size: Option<String>,
    pub cap_crew: Option<i32>,
    pub cap_passenger: Option<i32>,
    pub terrain: Option<Vec<String>>,
    pub pace: Option<i32>,
    pub speed: Option<String>,
}

impl From<&Vehicle> for VehicleSummary {
    fn from(vehicle: &Vehicle) -> Self {
        let speed_str = vehicle.speed.as_ref().map(|s| {
            let mut speeds = Vec::new();
            if let Some(walk) = s.walk {
                speeds.push(format!("Walk {}", walk));
            }
            if let Some(swim) = s.swim {
                speeds.push(format!("Swim {}", swim));
            }
            if let Some(fly) = s.fly {
                speeds.push(format!("Fly {}", fly));
            }
            if speeds.is_empty() && vehicle.pace.is_some() {
                format!("Pace {}", vehicle.pace.unwrap())
            } else {
                speeds.join(", ")
            }
        }).or_else(|| {
            vehicle.pace.map(|p| format!("Pace {}", p))
        });
        
        VehicleSummary {
            name: vehicle.name.clone(),
            source: vehicle.source.clone(),
            vehicle_type: vehicle.vehicle_type.clone(),
            size: vehicle.size.clone(),
            cap_crew: vehicle.cap_crew,
            cap_passenger: vehicle.cap_passenger,
            terrain: vehicle.terrain.clone(),
            pace: vehicle.pace,
            speed: speed_str,
        }
    }
}