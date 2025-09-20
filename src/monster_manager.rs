// monster_manager.rs
// This file is responsible for saving and loading monster data to and from JSON files.
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Represents a single attack for a monster.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attack {
    pub attack_name: String,
    pub ability_used: String, // e.g., "str", "dex"
    pub dice_used: String,    // e.g., "d6", "d10"
    pub num_dice: i32,
    pub num_attacks: i32,
}

/// Represents the data for a single monster.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub ac: i32,
    pub pb: i32, // Proficiency Bonus
    pub str_mod: i32,
    pub dex_mod: i32,
    pub con_mod: i32,
    pub int_mod: i32,
    pub wis_mod: i32,
    pub cha_mod: i32,
    pub exp: i32, // XP value
    pub attacks: Vec<Attack>,
}

const MONSTER_DIR: &str = "monsters";


pub fn check_for_monsters() -> bool{
    // Define the path to the "Monsters" directory relative to the executable
    let mut path = std::env::current_exe().unwrap_or_else(|_| Path::new(".").to_path_buf());
    path.pop(); // Go up one level from the executable to the target/debug or target/release directory
    path.push("Monsters"); // Append the "Monsters" directory

    println!("Checking for monsters directory at: {:?}", path);
    path.exists()
}

pub fn save_monster(monster: Monster) -> io::Result<()> {
    // Ensure the monsters directory exists
    if !Path::new(MONSTER_DIR).exists() {
        fs::create_dir(MONSTER_DIR)?;
    }

    let file_path = format!("{}/{}.json", MONSTER_DIR, monster.name);
    let json = serde_json::to_string_pretty(&monster)?;
    let mut file = fs::File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Reads a monster from a JSON file.
pub fn read_monster(monster_name: &str) -> io::Result<Monster> {
    let file_path = format!("{}/{}.json", MONSTER_DIR, monster_name);
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let monster: Monster = serde_json::from_str(&contents)?;
    Ok(monster)
}

/// Reads all monster files from the directory and returns a vector of Monster structs.
pub fn read_all_monsters() -> Vec<Monster> {
    let mut monsters = Vec::new();
    if let Ok(entries) = fs::read_dir(MONSTER_DIR) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(monster) = read_monster(&path.file_stem().unwrap().to_string_lossy()) {
                        monsters.push(monster);
                    }
                }
            }
        }
    }
    monsters
}

/// Deletes a monster's JSON file.
pub fn delete_monster(monster_name: &str) -> io::Result<()> {
    let file_path = format!("{}/{}.json", MONSTER_DIR, monster_name);
    fs::remove_file(file_path)?;
    Ok(())
}

/// Adds an attack to an existing monster and saves the updated monster.
pub fn add_attack_to_monster(monster_name: &str, new_attack: Attack) -> io::Result<()> {
    let mut monster = read_monster(monster_name)?;
    monster.attacks.push(new_attack);
    save_monster(monster)?;
    Ok(())
}

/// Deletes an attack from an existing monster and saves the updated monster.
pub fn delete_attack_from_monster(monster_name: &str, attack_name: &str) -> io::Result<()> {
    let mut monster = read_monster(monster_name)?;
    monster.attacks.retain(|attack| attack.attack_name != attack_name);
    save_monster(monster)?;
    Ok(())
}
