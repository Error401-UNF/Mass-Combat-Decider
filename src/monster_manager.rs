// monster_manager.rs
//
// This file is responsible for all data management operations related to monsters.
// It handles reading from and writing to the "Monsters" directory.

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

// Represents the data structure for a monster.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub ac: i32,
    pub exp: i32,
    pub pb: i32,
    pub str_mod: i32,
    pub dex_mod: i32,
    pub con_mod: i32,
    pub int_mod: i32,
    pub wis_mod: i32,
    pub cha_mod: i32,
    pub attacks: Vec<Attack>,
}

// Represents the data structure for an attack.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attack {
    pub attack_name: String,
    pub ability_used: String,
    pub dice_used: String,
    pub num_dice: i32,
    pub num_attacks: i32,
    pub saving_throw: bool,
}

/// Checks if the "Monsters" directory exists.
/// This is used to determine if a new user should be shown the welcome screen.
pub fn check_for_monsters() -> bool {
    // Define the path to the "Monsters" directory relative to the executable
    let mut path = get_base_path().unwrap();// Go up one level from the executable to the target/debug or target/release directory
    path.push("Monsters"); // Append the "Monsters" directory

    println!("Checking for monsters directory at: {:?}", path);
    path.exists()
}

pub fn get_base_path() -> io::Result<PathBuf> {
    let mut path = std::env::home_dir().unwrap();

    if cfg!(target_os = "windows") {
        // Windows path: C:\Users\Name\Documents\MonsterMan
        path.push("Documents");
    } else {
        // Unix path: /home/name/.config/MonsterMan
        path.push(".config");
    }

    path.push("MonsterMan");
    Ok(path)
}


/// Saves a monster to a JSON file.
pub fn save_monster(monster: Monster) -> io::Result<()> {
    // Ensure the Monsters directory exists.
    let mut path = get_base_path()?;
    path.push("Monsters");
    if !path.exists() {
        fs::create_dir(&path)?;
    }

    // Create the file path for the new monster.
    path.push(format!("{}.json", monster.name));

    let json_data = serde_json::to_string_pretty(&monster)?;
    let mut file = File::create(&path)?;
    file.write_all(json_data.as_bytes())?;

    println!("Saved monster to file: {:?}", path);
    Ok(())
}

/// Reads a monster's data from a JSON file by name.
pub fn read_monster(monster_name: &str) -> Option<Monster> {
    let mut path = match get_base_path() {
        Ok(p) => p,
        Err(_) => return None,
    };
    path.push("Monsters");
    path.push(format!("{}.json", monster_name));

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }

    match serde_json::from_str(&contents) {
        Ok(monster) => Some(monster),
        Err(e) => {
            eprintln!("Failed to parse monster JSON for '{}': {}", monster_name, e);
            None
        }
    }
}

/// Reads all monsters from the "Monsters" directory.
pub fn read_all_monsters() -> Vec<Monster> {
    let mut path = match get_base_path() {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    path.push("Monsters");

    let mut monsters = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let monster_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
                if let Some(monster) = read_monster(monster_name) {
                    monsters.push(monster);
                }
            }
        }
    }
    monsters.sort_by( |a,b| a.name.cmp(&b.name));
    monsters
}

/// Deletes a monster's JSON file.
pub fn delete_monster(monster_name: &str) -> io::Result<()> {
    let mut path = get_base_path()?;
    path.push("Monsters");
    path.push(format!("{}.json", monster_name));
    fs::remove_file(&path)?;
    println!("Deleted monster file: {:?}", path);
    Ok(())
}

/// Adds a new attack to an existing monster.
pub fn add_attack_to_monster(monster_name: &str, new_attack: Attack) -> io::Result<()> {
    let mut monster_data = read_monster(monster_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Monster not found"))?;

    monster_data.attacks.push(new_attack);

    save_monster(monster_data)
}

/// Deletes an attack from a monster by name.
pub fn delete_attack_from_monster(monster_name: &str, attack_name: &str) -> io::Result<()> {
    let mut monster_data = read_monster(monster_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Monster not found"))?;
    
    let original_len = monster_data.attacks.len();
    monster_data.attacks.retain(|a| a.attack_name != attack_name);
    
    if monster_data.attacks.len() < original_len {
        save_monster(monster_data)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Attack not found"))
    }
}
