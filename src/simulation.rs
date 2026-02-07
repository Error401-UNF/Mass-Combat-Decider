// simulation.rs

use gtk::{prelude::*, Adjustment, Align, Box, Button, FlowBox, Frame, Label, ListBox, Orientation, ScrolledWindow, SpinButton, TextView, TextBuffer};
use libadwaita::{Application as AdwApplication, Window as AdwWindow};
use libadwaita::prelude::AdwWindowExt;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::rc::Rc; 
use std::cell::RefCell;
use rand::Rng;
use chrono; 

use crate::monster_manager::{Attack, get_base_path};

use super::monster_manager::{self, Monster};
use super::ui_manager; // Import ui_manager to switch back to monster list

/// A struct to hold the data for each individual combatant instance.
#[derive(Clone,Debug, serde::Serialize,serde::Deserialize)]
struct Combatant {
    instance_name: String,
    monster_template: Monster,
    current_hp: i32,
    max_hp: i32,
}

/// A struct to hold the shared state of the simulation.
#[derive(Clone,Debug)]
pub struct SimulationState {
    combatants: Rc<RefCell<Vec<Combatant>>>,
    killed_monsters: Rc<RefCell<Vec<Combatant>>>,
    flow_box: FlowBox,
    console_buffer: Rc<RefCell<TextBuffer>>,
    console_text_view: TextView,
}
#[derive(Clone,Debug, serde::Serialize, serde::Deserialize)]
pub struct StaticSimulationState {
    combatants: Vec<Combatant>,
    killed_monsters: Vec<Combatant>,
}

impl StaticSimulationState {
    fn make_static(simulation_state: &SimulationState) -> Self {
        let combatants = simulation_state.combatants.borrow().clone();
        let killed_monsters = simulation_state.killed_monsters.borrow().clone();
        Self {
            combatants: combatants,
            killed_monsters: killed_monsters,
        }
    }
    fn replace_with_static(self, simulation_state: &SimulationState) {
        let mut combatants = simulation_state.combatants.borrow_mut();
        let mut killed = simulation_state.killed_monsters.borrow_mut();

        // assume combattents and killed are empty, just fill them idk
        for mon in self.combatants{
            combatants.push(mon);
        }
        for mon in self.killed_monsters {
            killed.push(mon)
        }
    
    }
}

/// This function creates the modal window for selecting monsters and their quantities.
pub fn show_simulation_setup_menu(app: &AdwApplication, parent_window: &AdwWindow) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Setup Simulation")
        .transient_for(parent_window)
        .modal(true)
        .default_width(450)
        .default_height(500)
        .build();

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("Select Combatants")
        .halign(Align::Center)
        .build();
    title.add_css_class("title-3");
    main_vbox.append(&title);

    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    
    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");

    let all_monsters = monster_manager::read_all_monsters();
    // A vector to hold the spin buttons to retrieve their values later.
    let mut spin_buttons: Vec<(SpinButton, Monster)> = Vec::new();

    if all_monsters.is_empty() {
        list_box.append(&Label::new(Some("No monsters exist. Please create one first.")));
    } else {
        for monster in all_monsters {
            let row = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(6)
                .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
                .build();
            
            let name_label = Label::builder()
                .label(&monster.name)
                .halign(Align::Start)
                .hexpand(true)
                .build();
            
            // Adjustment for the spin button (0 to 100, step 1)
            let adj = Adjustment::new(0.0, 0.0, 100.0, 1.0, 5.0, 0.0);
            let spin_button = SpinButton::builder()
                .adjustment(&adj)
                .numeric(true)
                .build();
            
            row.append(&name_label);
            row.append(&spin_button);
            list_box.append(&row);

            // Store the spin button and its corresponding monster
            spin_buttons.push((spin_button, monster));
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let start_button = Button::builder()
        .label("Start Simulation")
        .halign(Align::End)
        .build();
    start_button.add_css_class("suggested-action");
    main_vbox.append(&start_button);

    // --- Start Button Logic ---
    let window_clone = window.clone();
    let app_clone = app.clone();
    let parent_window_clone = parent_window.clone();
    start_button.connect_clicked(move |_| {
        let mut selected_monsters: Vec<(Monster, i32)> = Vec::new();
        for (spin_button, monster) in &spin_buttons {
            let count = spin_button.value() as i32;
            if count > 0 {
                selected_monsters.push((monster.clone(), count));
            }
        }

        if selected_monsters.is_empty() {
            println!("No monsters selected for the simulation.");
            return;
        }

        // Close the setup modal
        window_clone.close();
        // Call the function to build the main simulation view
        start_simulation_view(&app_clone, &parent_window_clone, selected_monsters);
    });

    window.set_content(Some(&main_vbox));
    window.present();
}

/// This function takes the selected monsters and builds the main card view.
pub fn start_simulation_view(app: &AdwApplication, window: &AdwWindow, selected_monsters: Vec<(Monster, i32)>) {
    window.set_title(Some("Mass Combat Decider - Simulation"));

    // --- Generate the list of individual combatants ---
    let mut combatants: Vec<Combatant> = Vec::new();
    // Use a HashMap to track counts for numbering (e.g., Goblin 1, Goblin 2)
    let mut name_counts: HashMap<String, i32> = HashMap::new();

    for (monster, count) in selected_monsters {
        for _ in 0..count {
            let current_count = name_counts.entry(monster.name.clone()).or_insert(0);
            *current_count += 1;
            
            let instance_name = if count > 1 {
                format!("{} {}", monster.name, current_count)
            } else {
                monster.name.clone()
            };

            combatants.push(Combatant {
                instance_name,
                current_hp: monster.hp, // Set initial HP
                monster_template: monster.clone(),
                max_hp: monster.hp
            });
        }
    }

    // --- Build the UI ---
    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .vexpand(true) // Make the main_vbox expand vertically
        .hexpand(true) // Make the main_vbox expand horizontally
        .build();

    // Add a title for the simulation view
    let simulation_title = Label::builder()
        .label("Live Simulation")
        .halign(Align::Center)
        .margin_bottom(12)
        .build();
    simulation_title.add_css_class("title-1"); // Use a larger title class
    main_vbox.append(&simulation_title);

    // --- Console Output Section ---
    let console_scrolled_window = ScrolledWindow::builder()
        .height_request(150)
        .hexpand(true)
        .margin_bottom(12)
        .build();

    let console_text_view = TextView::builder()
        .editable(false)
        .wrap_mode(gtk::WrapMode::Word)
        .build();
    
    let console_buffer = Rc::new(RefCell::new(console_text_view.buffer()));
    console_scrolled_window.set_child(Some(&console_text_view));
    main_vbox.append(&console_scrolled_window);

    if let Ok(buffer) = console_buffer.try_borrow_mut() {
        buffer.insert(&mut buffer.start_iter(), "Simulation console: Last 50 lines will be displayed here.\n");
    }

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_vexpand_set(true);

    let flow_box = FlowBox::builder()
        .valign(Align::Start)
        .max_children_per_line(4)
        .min_children_per_line(1)
        .selection_mode(gtk::SelectionMode::None)
        .row_spacing(12) // Use row_spacing and column_spacing for FlowBox
        .column_spacing(12)
        .margin_top(12).margin_bottom(12).margin_start(12).margin_end(12)
        .build();

    let shared_state = Rc::new(RefCell::new(combatants));
    
    
    let simulation_state = SimulationState {
        combatants: Rc::clone(&shared_state),
        killed_monsters: Rc::new(RefCell::new(Vec::new())),
        flow_box: flow_box.clone(),
        console_buffer: Rc::clone(&console_buffer),
        console_text_view: console_text_view.clone(),
    };

    // check if simulation file exsists if so set simulation state to that
    if check_for_simulation() {
        let static_sim = get_simulation().unwrap();
        let _ = remove_simulation_file();
        static_sim.replace_with_static(&simulation_state);
    }




    // Populate the FlowBox with cards from the initial combatant list
    for combatant in shared_state.borrow().iter() {
        let card = create_combatant_card(combatant, &simulation_state);
        simulation_state.flow_box.insert(&card, -1);
    }
    
    // --- Buttons Section ---
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Center)
        .margin_bottom(12)
        .build();
    
    // Add "See Killed" button
    let killed_button = Button::builder()
        .label("See Killed")
        .build();
    killed_button.add_css_class("see-killed-action");
    let app_clone = app.clone();
    let window_clone_killed = window.clone();
    let simulation_state_clone = simulation_state.clone();
    killed_button.connect_clicked(move |_| {
        show_killed_monsters_menu(&app_clone, &window_clone_killed, simulation_state_clone.clone());
    });
    button_box.append(&killed_button);

    // Add Edit Simulation button
    let edit_button = Button::builder()
        .label("Edit Simulation")
        .build();
    edit_button.add_css_class("edit-action");

    let app_clone = app.clone();
    let window_clone_edit = window.clone();
    let simulation_state_clone = simulation_state.clone();
    edit_button.connect_clicked(move |_| {
        show_edit_simulation_menu(&app_clone, &window_clone_edit, simulation_state_clone.clone());
    });
    button_box.append(&edit_button);

    // Add Exit Simulation button
    let exit_button = Button::builder()
        .label("Exit Simulation")
        .build();
    exit_button.add_css_class("destructive-action");

    let app_clone_exit = app.clone();
    let window_clone_exit = window.clone();
    exit_button.connect_clicked(move |_| {
        ui_manager::switch_to_monster_list(&app_clone_exit, &window_clone_exit);
    });
    button_box.append(&exit_button);

    // Add Save Simulation button
    let save_button = Button::builder()
        .label("Save Simulation")
        .build();
    save_button.add_css_class("suggested-action");

    let app_clone_save = app.clone();
    let window_clone_save = window.clone();
    let simulation_state_clone = simulation_state.clone();
    save_button.connect_clicked(move |_| {
        let _ = self::save_simulation(&simulation_state_clone);
        ui_manager::switch_to_monster_list(&app_clone_save, &window_clone_save);
    });
    button_box.append(&save_button);

    main_vbox.append(&button_box);
    scrolled_window.set_child(Some(&simulation_state.flow_box));
    main_vbox.append(&scrolled_window);
    window.set_content(Some(&main_vbox));
}

/// This function creates and shows the modal for killed monsters.
fn show_killed_monsters_menu(app: &AdwApplication, parent_window: &AdwWindow, simulation_state: SimulationState) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Killed Monsters")
        .transient_for(parent_window)
        .modal(true)
        .default_width(450)
        .default_height(500)
        .build();

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("Killed Monsters and Total XP")
        .halign(Align::Center)
        .build();
    title.add_css_class("title-3");
    main_vbox.append(&title);

    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    
    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");

    let killed_monsters = simulation_state.killed_monsters.borrow();
    let mut total_xp = 0;

    if killed_monsters.is_empty() {
        list_box.append(&Label::new(Some("No monsters have been killed yet.")));
    } else {
        for combatant in killed_monsters.iter() {
            let row = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(6)
                .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
                .build();
            
            let name_label = Label::builder()
                .label(&format!("{} ({} XP)", combatant.instance_name, combatant.monster_template.exp))
                .halign(Align::Start)
                .hexpand(true)
                .build();
            
            row.append(&name_label);
            list_box.append(&row);
            total_xp += combatant.monster_template.exp;
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let xp_label = Label::builder()
        .label(&format!("Total XP: {}", total_xp))
        .halign(Align::End)
        .build();
    xp_label.add_css_class("title-4");
    xp_label.add_css_class("suggested-action");
    main_vbox.append(&xp_label);

    let close_button = Button::builder()
        .label("Close")
        .halign(Align::End)
        .build();
    close_button.add_css_class("destructive-action");
    let window_clone = window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });
    main_vbox.append(&close_button);

    window.set_content(Some(&main_vbox));
    window.present();
}


pub fn show_edit_simulation_menu(app: &AdwApplication, parent_window: &AdwWindow, simulation_state: SimulationState) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Edit Simulation")
        .transient_for(parent_window)
        .modal(true)
        .default_width(450)
        .default_height(500)
        .build();

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("Edit Combatants")
        .halign(Align::Center)
        .build();
    title.add_css_class("title-3");
    main_vbox.append(&title);

    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    
    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");

    let all_monsters = monster_manager::read_all_monsters();
    let mut spin_buttons: Vec<(SpinButton, Monster)> = Vec::new();
    let initial_counts: HashMap<String, i32> = {
        let combatants = simulation_state.combatants.borrow();
        let mut counts = HashMap::new();
        for combatant in combatants.iter() {
            *counts.entry(combatant.monster_template.name.clone()).or_insert(0) += 1;
        }
        counts
    };

    if all_monsters.is_empty() {
        list_box.append(&Label::new(Some("No monsters exist. Please create one first.")));
    } else {
        for monster in all_monsters {
            let row = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(6)
                .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
                .build();
            
            let name_label = Label::builder()
                .label(&monster.name)
                .halign(Align::Start)
                .hexpand(true)
                .build();
            
            let initial_value = *initial_counts.get(&monster.name).unwrap_or(&0) as f64;
            let adj = Adjustment::new(initial_value, 0.0, 100.0, 1.0, 5.0, 0.0);
            let spin_button = SpinButton::builder()
                .adjustment(&adj)
                .numeric(true)
                .build();
            
            row.append(&name_label);
            row.append(&spin_button);
            list_box.append(&row);
            spin_buttons.push((spin_button, monster));
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let start_button = Button::builder()
        .label("Update Simulation")
        .halign(Align::End)
        .build();
    start_button.add_css_class("suggested-action");
    main_vbox.append(&start_button);

    let window_clone = window.clone();
    start_button.connect_clicked(move |_| {
        let mut selected_monsters: Vec<(Monster, i32)> = Vec::new();
        for (spin_button, monster) in &spin_buttons {
            let count = spin_button.value() as i32;
            if count > 0 {
                selected_monsters.push((monster.clone(), count));
            }
        }
        window_clone.close();
        update_simulation_view(&selected_monsters, &simulation_state);
    });

    window.set_content(Some(&main_vbox));
    window.present();
}

/// Updates the simulation view with a new set of monsters, preserving existing combatant state.
fn update_simulation_view(selected_monsters: &Vec<(Monster, i32)>, simulation_state: &SimulationState) {
    let mut current_combatants = simulation_state.combatants.borrow_mut();
    
    // Create a new HashMap to group the current combatants by template name.
    let mut existing_combatants_map: HashMap<String, Vec<Combatant>> = HashMap::new();
    for combatant in current_combatants.drain(..) {
        existing_combatants_map.entry(combatant.monster_template.name.clone()).or_insert(Vec::new()).push(combatant);
    }

    let mut new_combatant_list: Vec<Combatant> = Vec::new();
    
    // Iterate through the new list of desired monsters and rebuild the combatant list.
    for (monster_template, desired_count) in selected_monsters {
        let monster_name = &monster_template.name;
        
        // Get the existing combatants of this type, sort them by name for predictable retention.
        let mut existing_of_type = existing_combatants_map.remove(monster_name).unwrap_or_else(Vec::new);
        existing_of_type.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));

        let num_existing = existing_of_type.len();
        let num_needed = *desired_count as usize;

        let mut existing_counter = 0;
        let mut _new_counter = 0;
        
        for _ in 0..num_needed {
            // Check if we can reuse an existing combatant
            if existing_counter < num_existing {
                let combatant_to_keep = existing_of_type.remove(0);
                new_combatant_list.push(combatant_to_keep);
                existing_counter += 1;
            } else {
                // If not, we need to find the highest number used and create a new combatant.
                let mut max_number = 0;
                for existing_c in new_combatant_list.iter() {
                    if existing_c.monster_template.name == *monster_name {
                        if let Some(num_str) = existing_c.instance_name.split(' ').last() {
                            if let Ok(num) = num_str.parse::<i32>() {
                                if num > max_number {
                                    max_number = num;
                                }
                            }
                        }
                    }
                }
                _new_counter = max_number + 1;

                let instance_name = if *desired_count > 1 {
                    format!("{} {}", monster_name, _new_counter)
                } else {
                    monster_name.clone()
                };

                new_combatant_list.push(Combatant {
                    instance_name,
                    current_hp: monster_template.hp,
                    monster_template: monster_template.clone(),
                    max_hp: monster_template.hp,
                });
            }
        }
    }
    
    // Update the shared state with the new list of combatants.
    *current_combatants = new_combatant_list;
    
    // Clear and rebuild the UI
    while let Some(child) = simulation_state.flow_box.first_child() {
        simulation_state.flow_box.remove(&child);
    }

    // Now, repopulate the FlowBox with the updated list.
    for combatant in current_combatants.iter() {
        let card = create_combatant_card(combatant, simulation_state);
        simulation_state.flow_box.insert(&card, -1);
    }
}
/// Helper function to calculate damage rolls and format the output.
/// Returns a tuple of (total_damage, formatted_damage_string).
fn calculate_damage(num_dice: i32, dice_used: &str, ability_mod: i32) -> (i32, String) {
    let mut rng = rand::rngs::ThreadRng::default();
    let dice_value_str = dice_used.trim_start_matches('d');
    let max_dice_val: i32 = dice_value_str.parse().unwrap_or(1);
    let mut total_damage = 0;
    let mut damage_rolls = Vec::new(); // Store individual rolls

    for _ in 0..num_dice {
        let roll = rng.random_range(1..=max_dice_val);
        damage_rolls.push(roll.to_string()); // Convert to string for joining
        total_damage += roll;
    }

    let mut damage_output = String::new();
    if !damage_rolls.is_empty() {
        damage_output.push_str(&damage_rolls.join(" + "));
    }

    // Add ability modifier
    if ability_mod != 0 || damage_rolls.is_empty() { // Always show if it's not zero, or if there were no dice
        if !damage_rolls.is_empty() { // Add " + " only if there were previous dice rolls
            damage_output.push_str(" + ");
        }
        damage_output.push_str(&ability_mod.to_string());
    }
    
    total_damage += ability_mod; // Add ability modifier to the total damage

    damage_output.push_str(&format!(" = {}", total_damage));

    (total_damage, damage_output)
}


/// Helper function to create a single monster card for the simulation view.
fn create_combatant_card(combatant: &Combatant, simulation_state: &SimulationState) -> Frame {
    let card_frame = Frame::builder()
        .label(&combatant.instance_name)
        .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
        .build();
    
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
        .build();

    // --- Header with name and Kill button ---
    let header_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    
    let name_label = Label::builder()
        .label(&combatant.instance_name)
        .halign(Align::Start)
        .hexpand(true)
        .build();
    name_label.add_css_class("title-4");

    let kill_button = Button::builder()
        .label("Kill")
        .halign(Align::End)
        .build();
    kill_button.add_css_class("destructive-action");
    
    // Add the kill button functionality
    let card_frame_clone = card_frame.clone();
    let combatants_clone = Rc::clone(&simulation_state.combatants);
    let killed_monsters_clone = Rc::clone(&simulation_state.killed_monsters);
    let combatant_instance_name = combatant.instance_name.clone();
    let _combatant_to_kill = combatant.clone();
    let flow_box_clone = simulation_state.flow_box.clone();

    kill_button.connect_clicked(move |_| {
        // Find and remove the combatant from the shared state
        if let Ok(mut combatants) = combatants_clone.try_borrow_mut() {
            if let Some(pos) = combatants.iter().position(|c| c.instance_name == combatant_instance_name) {
                let killed = combatants.remove(pos);
                killed_monsters_clone.borrow_mut().push(killed);
            }
        }
        // Remove the card from the UI
        flow_box_clone.remove(&card_frame_clone);
    });

    header_box.append(&name_label);
    header_box.append(&kill_button);
    vbox.append(&header_box);

    // --- HP and AC Box ---
    let stats_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    
    let hp_label = Label::new(Some("HP:"));
    let hp_adj = Adjustment::new(combatant.current_hp as f64, 0.0, 9999.0, 1.0, 10.0, 0.0);
    let hp_spin_button = SpinButton::builder()
        .adjustment(&hp_adj)
        .numeric(true)
        .build();
    let max_hp_label = Label::new(Some(&format!("Max HP: {}",combatant.max_hp)));


    // Update the combatant's HP in the shared state when the spin button value changes
    let combatants_clone = Rc::clone(&simulation_state.combatants);
    let combatant_instance_name_clone = combatant.instance_name.clone();
    hp_spin_button.connect_value_changed(move |btn| {
        if let Ok(mut combatants) = combatants_clone.try_borrow_mut() {
            if let Some(c) = combatants.iter_mut().find(|c| c.instance_name == combatant_instance_name_clone) {
                c.current_hp = btn.value() as i32;
            }
        }
    });
    
    let ac_label = Label::new(Some(&format!("AC: {}", combatant.monster_template.ac)));

    stats_box.append(&hp_label);
    stats_box.append(&hp_spin_button);
    stats_box.append(&max_hp_label);
    stats_box.append(&ac_label);
    vbox.append(&stats_box);

    // --- Saves Section ---
    let saves_label = Label::builder()
        .label("<b>Saves</b>")
        .use_markup(true)
        .halign(Align::Start)
        .margin_top(6)
        .build();
    vbox.append(&saves_label);

    let saves_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    
    let stats = ["Str", "Dex", "Con", "Int", "Wis", "Cha"];
    for stat_name in stats.iter() {
        let save_button = Button::builder()
            .label(*stat_name)
            .build();
        
        let combatant_clone = combatant.clone();
        let console_buffer_clone = Rc::clone(&simulation_state.console_buffer);
        let console_text_view_clone = simulation_state.console_text_view.clone();
        
        // Clone the stat_name string to move it into the closure, fixing the lifetime issue.
        let stat_name_clone = stat_name.to_string();

        save_button.connect_clicked(move |_| {
            let mut rng = rand::rngs::ThreadRng::default();
            let d20_roll = rng.random_range(1..=20);
            
            let modifier = match stat_name_clone.to_lowercase().as_str() {
                "str" => combatant_clone.monster_template.str_mod,
                "dex" => combatant_clone.monster_template.dex_mod,
                "con" => combatant_clone.monster_template.con_mod,
                "int" => combatant_clone.monster_template.int_mod,
                "wis" => combatant_clone.monster_template.wis_mod,
                "cha" => combatant_clone.monster_template.cha_mod,
                _ => 0,
            };
            
            let total = d20_roll + modifier;
            
            if let Ok(buffer) = console_buffer_clone.try_borrow_mut() {
                let mut iter = buffer.end_iter();
                let message = format!("{}: {} rolled a {} Save: {} (1d20) + {} (Mod) = {}\n",
                    chrono::Local::now().format("%H:%M:%S"),
                    combatant_clone.instance_name,
                    stat_name_clone,
                    d20_roll,
                    modifier,
                    total
                );
                buffer.insert(&mut iter, &message);
                
                let line_count = buffer.line_count();
                if line_count > 50 {
                    let lines_to_remove = line_count - 50;
                    let mut start_iter = buffer.start_iter();
                    start_iter.forward_lines(lines_to_remove);
                    buffer.delete(&mut buffer.start_iter(), &mut start_iter);
                }
            }
            if let Some(adj) = console_text_view_clone.vadjustment() {
                adj.set_value(adj.upper());
            }
        });
        
        saves_box.append(&save_button);
    }
    vbox.append(&saves_box);

    // --- Attacks List ---
    if !combatant.monster_template.attacks.is_empty() {
        let attacks_label = Label::builder()
            .label("<b>Attacks</b>")
            .use_markup(true)
            .halign(Align::Start)
            .margin_top(6)
            .build();
        vbox.append(&attacks_label);

        for attack in &combatant.monster_template.attacks {
            let attack_hbox = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(6)
                .build();

            let save_dc = 8 + get_ability_mod(combatant, attack) + combatant.monster_template.pb;

            let attack_details = if ! attack.saving_throw {
                format!(
                "• {} ({}{}, {}/turn)",
                attack.attack_name,
                attack.num_dice,
                attack.dice_used,
                attack.num_attacks,
                )
            } else {
                format!(
                "• {} ({}{}, DC {})",
                attack.attack_name,
                attack.num_dice,
                attack.dice_used,
                save_dc,
                )
            };

            let attack_label = Label::builder()
                .label(&attack_details)
                .halign(Align::Start)
                .hexpand(true) // Allow label to expand
                .build();
            attack_hbox.append(&attack_label);

            // "Use" button for each attack
            let use_button = Button::builder()
                .label("Use")
                .build();

            // Clone necessary variables for the closure
            let combatant_clone = combatant.clone();
            let attack_clone = attack.clone();
            let console_buffer_clone = Rc::clone(&simulation_state.console_buffer);
            let console_text_view_clone = simulation_state.console_text_view.clone();

            use_button.connect_clicked(move |_| {
                let creature_name = combatant_clone.instance_name.clone();
                let attack_name = attack_clone.attack_name.clone();
                let attacks_per_turn = attack_clone.num_attacks;

                if let Ok(buffer) = console_buffer_clone.try_borrow_mut() {
                    let mut iter = buffer.end_iter();

                    if ! attack_clone.saving_throw {
                        buffer.insert(&mut iter, &format!("{}: {} started an attack using {} {} times.\n",
                            chrono::Local::now().format("%H:%M:%S"), creature_name, attack_name, attacks_per_turn));

                        for i in 0..attacks_per_turn {
                            // Calculate To Hit
                            let mut rng = rand::rngs::ThreadRng::default();
                            let d20_roll = rng.random_range(1..=20);
                            let ability_mod = get_ability_mod(&combatant_clone, &attack_clone);
                            let to_hit = d20_roll + ability_mod + combatant_clone.monster_template.pb;

                            // Check for a natural 20
                            let crit_message = if d20_roll == 20 { " -> CRITICAL HIT!" } else { "" };

                            // Calculate Damage using the new helper function
                            let (_total_damage, damage_output) = calculate_damage(
                                if d20_roll == 20 {attack_clone.num_dice*2} else {attack_clone.num_dice},
                                &attack_clone.dice_used,
                                ability_mod,
                            );

                            buffer.insert(&mut iter, &format!("  Attack {}: To hit: {} + {} (Total Mod) = {}{}; Damage: {}\n", 
                                i + 1, d20_roll, ability_mod + combatant_clone.monster_template.pb, to_hit, crit_message, damage_output));
                        }
                    } else {
                        buffer.insert(&mut iter, &format!("{}: {} started an attack using {}",
                            chrono::Local::now().format("%H:%M:%S"), creature_name, attack_name));

                        let (_total_damage, damage_output) = calculate_damage(
                            attack_clone.num_dice,
                            &attack_clone.dice_used,
                            0,
                        );
                        buffer.insert(&mut iter, &format!("  Damage: {}\n", damage_output));
                    }
                    
                    
                    
                    // --- Updated console limiting logic ---
                    let line_count = buffer.line_count();
                    if line_count > 50 {
                        let lines_to_remove = line_count - 50;
                        let mut start_iter = buffer.start_iter();
                        start_iter.forward_lines(lines_to_remove);
                        buffer.delete(&mut buffer.start_iter(), &mut start_iter);
                    }
                }

                // --- Scroll to the end ---
                // Safely get the adjustment from the text view
                if let Some(adj) = console_text_view_clone.vadjustment() {
                    // Scroll to the end of the text view
                    adj.set_value(adj.upper());
                }
            });
            attack_hbox.append(&use_button);
            vbox.append(&attack_hbox);
        }
    }

    card_frame.set_child(Some(&vbox));
    card_frame
}

fn save_simulation(simulation_state: &SimulationState) -> io::Result<()> {
    // Ensure the Monsters directory exists.
    let mut path = get_base_path()?;
    if !path.exists() {
        fs::create_dir(&path)?;
    }

    // Create the file path for the new monster.
    path.push(format!("active_simulation.json",));
    let static_sim = StaticSimulationState::make_static(simulation_state);
    let json_data = serde_json::to_string_pretty(&static_sim)?;
    let mut file = File::create(&path)?;
    file.write_all(json_data.as_bytes())?;

    //println!("Saved simulation to file: {:?}", path);
    Ok(())
}

fn get_simulation() -> Option<StaticSimulationState> {
    // gets and delets the saved simulation file
    let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop();
    path.push("active_simulation.json");

    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }

    match serde_json::from_str(&contents) {
        Ok(simulation) => Some(simulation),
        Err(e) => {
            eprintln!("Failed to parse monster JSON for active_simulation.json: {}", e);
            None
        }
    }
}

pub fn remove_simulation_file() -> io::Result<()>{
    // check if file exsists
    if check_for_simulation(){
        // kill it
        let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        path.pop();
        path.push("active_simulation.json");
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn check_for_simulation() -> bool {
    let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop(); // Go up one level from the executable to the target/debug or target/release directory
    path.push("active_simulation.json"); // Append the "Monsters" directory

    //println!("Checking for active simulation at: {:?}", path);
    path.exists()

}

fn get_ability_mod(combatant: &Combatant, attack: &Attack) -> i32 {
    let ability_mod = match attack.ability_used.as_str() {
        "str" => combatant.monster_template.str_mod,
        "dex" => combatant.monster_template.dex_mod,
        "con" => combatant.monster_template.con_mod,
        "int" => combatant.monster_template.int_mod,
        "wis" => combatant.monster_template.wis_mod,
        "cha" => combatant.monster_template.cha_mod,
        _ => 0,
    };
    ability_mod
}