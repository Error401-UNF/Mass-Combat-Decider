// monster_manager.rs
//
// This file is responsible for all data management operations related to monsters.
// It handles reading from and writing to the "Monsters" directory.

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

// Large scale monster folder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterFolder {
    pub folder_name: String,
    pub subfolders: Vec<MonsterFolder>,
    pub monsters: Vec<MonsterFile>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterFile {
    pub location: PathBuf,
    pub monster: Monster,
}


// Represents the data structure for a monster.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub ac: i32,
    pub exp: i32,
    pub pb: i32,
    pub speed: i32,
    pub hitdie: String,
    // mod order: str, dex, con, int, wis, cha
    pub mods: [i32;6],
    // save order: str, dex, con, int, wis, cha
    pub saves: [bool;6],
    pub vulnerabilities: Vec<String>,
    pub restistances: Vec<String>,
    pub immunities: Vec<String>,
    pub abilities: String,
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
        // Windows path: C:\Users\Name\AppData\Local\MonsterManager
        path.push("AppData");
        path.push("Local");
    } else {
        // Unix path: /home/name/.config/MonsterManager 
        path.push(".config");
    }

    path.push("MonsterManager");
    Ok(path)
}

pub fn read_all_monster_folders() -> Option<MonsterFolder> {
    let mut base_path = get_base_path().ok()?;
    base_path.push("Monsters");

    if !base_path.exists() {
        fs::create_dir_all(&base_path).ok()?;
    }

    read_monster_folder_recursive(&base_path)
}

fn read_monster_folder_recursive(dir_path: &Path) -> Option<MonsterFolder> {
    let folder_name = dir_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Monsters")
        .to_string();

    let mut subfolders = Vec::new();
    let mut monsters = Vec::new();

    let entries = fs::read_dir(dir_path).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            if let Some(subfolder) = read_monster_folder_recursive(&path) {
                subfolders.push(subfolder);
            }
        } else if path.extension().map_or(false, |ext| ext == "json") {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(monster) = serde_json::from_str::<Monster>(&contents) {
                    monsters.push(MonsterFile {
                        location: path,
                        monster,
                    });
                }
            }
        }
    }

    subfolders.sort_by(|a, b| a.folder_name.cmp(&b.folder_name));
    monsters.sort_by(|a, b| a.monster.name.cmp(&b.monster.name));

    Some(MonsterFolder {
        folder_name,
        subfolders,
        monsters,
    })
}

pub fn flatten_monster_folder(monster_folder: &MonsterFolder) -> Vec<MonsterFile> {
    let mut mon_vec: Vec<MonsterFile> = Vec::clone(&monster_folder.monsters);
    for subfolder in monster_folder.subfolders.clone() {
        mon_vec.append(&mut flatten_monster_folder(&subfolder).clone());
    }
    mon_vec
}


/// Finds and reads a single monster file by name, returning a `MonsterFile`.
pub fn read_monster(monster_name: &str) -> Option<MonsterFile> {
    let mut base_path = get_base_path().ok()?;
    base_path.push("Monsters");

    let location = find_monster_path(&base_path, monster_name)?;
    let contents = fs::read_to_string(&location).ok()?;
    let monster = serde_json::from_str(&contents).ok()?;

    Some(MonsterFile { location, monster })
}

/// Recursively searches for a `{monster_name}.json` path.
fn find_monster_path(dir_path: &Path, monster_name: &str) -> Option<PathBuf> {
    let entries = fs::read_dir(dir_path).ok()?;
    let target_filename = format!("{}.json", monster_name);

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found_path) = find_monster_path(&path, monster_name) {
                return Some(found_path);
            }
        } else if path.file_name().and_then(|s| s.to_str()) == Some(&target_filename) {
            return Some(path);
        }
    }
    None
}

/// Saves a `MonsterFile` directly to its stored `location`.
pub fn save_monster(monster_file: &MonsterFile) -> io::Result<()> {
    if let Some(parent) = monster_file.location.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let json_data = serde_json::to_string_pretty(&monster_file.monster)?;
    let mut file = File::create(&monster_file.location)?;
    file.write_all(json_data.as_bytes())?;

    println!("Saved monster to: {:?}", monster_file.location);
    Ok(())
}

/// Deletes a monster using the `location` path stored in `MonsterFile`.
pub fn delete_monster(monster_file: &MonsterFile) -> io::Result<()> {
    if monster_file.location.exists() {
        fs::remove_file(&monster_file.location)?;
        println!("Deleted monster file: {:?}", monster_file.location);
    }
    Ok(())
}
/// Adds a new attack to an existing monster.
pub fn add_attack_to_monster(monster_name: &str, new_attack: Attack) -> io::Result<()> {
    let mut monster_data = read_monster(monster_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Monster not found"))?;

    monster_data.monster.attacks.push(new_attack);

    save_monster(&monster_data)
}

/// Deletes an attack from a monster by name.
pub fn delete_attack_from_monster(monster_name: &str, attack_name: &str) -> io::Result<()> {
    let mut monster_data = read_monster(monster_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Monster not found"))?;
    
    let original_len = monster_data.monster.attacks.len();
    monster_data.monster.attacks.retain(|a| a.attack_name != attack_name);
    
    if monster_data.monster.attacks.len() < original_len {
        save_monster(&monster_data)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Attack not found"))
    }
}
