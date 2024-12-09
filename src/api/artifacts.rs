use std::fs;

use rocket::serde::json::Json;

use crate::types::artifact_types::SubStat;

#[get("/artifacts/substats")]
pub fn get_artifact_substats() -> Json<Vec<SubStat>> {
    let file = fs::read_to_string("assets/artifacts/substats.json").unwrap();
    let substats: Vec<SubStat> = serde_json::from_str(&file).unwrap();
    Json(substats)
}