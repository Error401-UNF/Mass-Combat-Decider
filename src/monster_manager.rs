use std::path::Path;
use std::fs; // Import fs for file system operations
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize,Clone)]
pub struct Monster {
    pub(crate) name:String,
    pub(crate) hp:i32,
    pub(crate) ac:i32,
    pub(crate) pb:i32,
    pub(crate) str_mod:i32,
    pub(crate) dex_mod:i32,
    pub(crate) con_mod:i32,
    pub(crate) int_mod:i32,
    pub(crate) wis_mod:i32,
    pub(crate) cha_mod:i32,

    #[serde(default)]
    pub(crate) attacks: Vec<Attack>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Attack {
    pub attack_name: String,
    pub ability_used: String,
    pub dice_used: String,
    pub num_dice: i32,
    pub num_attacks: i32,
}


pub fn check_for_monsters() -> bool{
    // Define the path to the "Monsters" directory relative to the executable
    let mut path = std::env::current_exe().unwrap_or_else(|_| Path::new(".").to_path_buf());
    path.pop(); // Go up one level from the executable to the target/debug or target/release directory
    path.push("Monsters"); // Append the "Monsters" directory

    println!("Checking for monsters directory at: {:?}", path);
    path.exists()
}

pub fn save_monster(new_mon: Monster) {
    // Define the path to the "Monsters" directory
    let mut dir_path = std::env::current_exe().unwrap_or_else(|_| Path::new(".").to_path_buf());
    dir_path.pop(); // Go up one level
    dir_path.push("Monsters"); // Append the "Monsters" directory

    // Create the directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&dir_path) {
        eprintln!("Failed to create monster directory {:?}: {}", dir_path, e);
        return;
    }

    // Define the file path for the monster
    let file_name = format!("{}.json", new_mon.name);
    let file_path = dir_path.join(&file_name);

    // Serialize the monster struct to a JSON string
    match serde_json::to_string_pretty(&new_mon) {
        Ok(json_string) => {
            // Write the JSON string to the file
            match fs::write(&file_path, json_string) {
                Ok(_) => println!("Monster '{}' saved successfully to {:?}", new_mon.name, file_path),
                Err(e) => eprintln!("Failed to write monster file {:?}: {}", file_path, e),
            }
        },
        Err(e) => eprintln!("Failed to serialize monster '{}': {}", new_mon.name, e),
    }
}

pub fn check_for_monster(mon_to_test: &Monster) -> bool{ // Changed to take a reference
    let mut path = std::env::current_exe().unwrap_or_else(|_| Path::new(".").to_path_buf());
    path.pop(); // Go up one level
    path.push("Monsters"); // Append the "Monsters" directory
    
    let test_str = format!("{}.json", mon_to_test.name);
    let test_path = path.join(&test_str);
    println!("Checking for monster file at: {:?}", test_path);
    test_path.exists()
}

fn get_monsters_dir_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap_or_else(|_| Path::new(".").to_path_buf());
    path.pop(); // Go up one level from the executable to the target/debug or target/release directory
    path.push("Monsters"); // Append the "Monsters" directory
    path
}

pub fn read_all_monsters() -> Vec<Monster> {
    let dir_path = get_monsters_dir_path();
    let mut monsters = Vec::new();

    if !dir_path.exists() {
        println!("Monsters directory does not exist: {:?}", dir_path);
        return monsters;
    }

    match fs::read_dir(&dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        match fs::read_to_string(&path) {
                            Ok(json_string) => {
                                match serde_json::from_str::<Monster>(&json_string) {
                                    Ok(monster) => {
                                        monsters.push(monster);
                                    },
                                    Err(e) => eprintln!("Failed to deserialize monster from {:?}: {}", path, e),
                                }
                            },
                            Err(e) => eprintln!("Failed to read file {:?}: {}", path, e),
                        }
                    }
                }
            }
        },
        Err(e) => eprintln!("Failed to read monsters directory {:?}: {}", dir_path, e),
    }
    return monsters;
}

pub fn delete_monster(monster_name: &str) -> Result<(), std::io::Error> {
    let dir_path = get_monsters_dir_path();
    let file_name = format!("{}.json", monster_name);
    let file_path = dir_path.join(&file_name);

    println!("Attempting to delete monster file: {:?}", file_path);
    fs::remove_file(&file_path)
}

pub fn add_attack_to_monster(monster_name: &str, new_attack: Attack) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = get_monsters_dir_path();
    let file_name = format!("{}.json", monster_name);
    let file_path = dir_path.join(&file_name);

    if !file_path.exists() {
        return Err(format!("Monster file not found: {:?}", file_path).into());
    }

    // Read the existing monster data
    let json_string = fs::read_to_string(&file_path)?;
    let mut monster: Monster = serde_json::from_str(&json_string)?;

    // Add the new attack
    monster.attacks.push(new_attack);

    // Serialize the updated monster data
    let updated_json_string = serde_json::to_string_pretty(&monster)?;

    // Write the updated data back to the file
    fs::write(&file_path, updated_json_string)?;

    println!("Attack added to '{}' and file updated successfully.", monster_name);
    Ok(())
}

pub fn delete_attack_from_monster(monster_name: &str, attack_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = get_monsters_dir_path();
    let file_name = format!("{}.json", monster_name);
    let file_path = dir_path.join(&file_name);

    if !file_path.exists() {
        return Err(format!("Monster file not found: {:?}", file_path).into());
    }

    // Read the existing monster data
    let json_string = fs::read_to_string(&file_path)?;
    let mut monster: Monster = serde_json::from_str(&json_string)?;

    // Find and remove the attack
    let original_len = monster.attacks.len();
    monster.attacks.retain(|a| a.attack_name != attack_name);

    if monster.attacks.len() < original_len {
        // Serialize the updated monster data
        let updated_json_string = serde_json::to_string_pretty(&monster)?;

        // Write the updated data back to the file
        fs::write(&file_path, updated_json_string)?;

        println!("Attack '{}' deleted from '{}' and file updated successfully.", attack_name, monster_name);
        Ok(())
    } else {
        Err(format!("Attack '{}' not found for monster '{}'.", attack_name, monster_name).into())
    }
}

pub fn read_monster(name: &str) -> Option<Monster> {
    let dir_path = get_monsters_dir_path();
    let file_path = dir_path.join(format!("{}.json", name));
    
    fs::read_to_string(&file_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}