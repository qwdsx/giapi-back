use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubStat {
    id: String,
    name: String,
    roll_values: Vec<f32>,
    max_roll: f32
}