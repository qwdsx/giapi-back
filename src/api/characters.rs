use std::fs;

use rocket::serde::json::Json;

use crate::types::character_types::{Character, CharacterRaw};

const BASE_URL: &str = "http://localhost:8000";

#[get("/characters")]
pub fn get_characters() -> Json<Vec<String>> {
    let file = fs::read_to_string("assets/characters/characters.json").unwrap();
    let mut characters: Vec<String> = serde_json::from_str(&file).unwrap();
    characters.sort();
    Json(characters)
}

#[get("/characters/<id>")]
pub fn get_character(id: &str) -> Json<Character> {
    Json(create_character_from_raw(id))
}

#[get("/characters?<id>")]
pub fn get_characters_by_search_params(id: &str) -> Json<Vec<String>> {
    let file = fs::read_to_string("assets/characters/characters.json").unwrap();
    let characters: Vec<String> = serde_json::from_str(&file).unwrap();
    let filtered_characters = characters
        .iter()
        .filter(|i| i.contains(id))
        .cloned()
        .collect::<Vec<String>>();

    let mut characters_final = Vec::<String>::new();
    
    for e in filtered_characters.iter() {
        characters_final.push(e.to_string());
    }

    characters_final.sort();

    Json(characters_final)
}


fn create_character_from_raw(id: &str) -> Character {
    let file = fs::read_to_string(format!("assets/characters/{}/{}.json", id, id)).unwrap();
    let icon_url = format!("{}/assets/characters/{}/ui-avataricon.png", BASE_URL, id);
    let character_raw: CharacterRaw = serde_json::from_str(&file).unwrap();
    Character::from_raw(&character_raw, icon_url)
}