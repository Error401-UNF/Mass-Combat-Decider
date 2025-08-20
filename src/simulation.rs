// simulation.rs

use gtk::{prelude::*, Adjustment, Align, Box, Button, FlowBox, Frame, Label, ListBox, Orientation, ScrolledWindow, SpinButton, TextView, TextBuffer};
use libadwaita::{Application as AdwApplication, Window as AdwWindow};
use libadwaita::prelude::AdwWindowExt;
use std::collections::HashMap;
use std::rc::Rc; // For shared ownership
use std::cell::RefCell; // For interior mutability
use rand::Rng;
use chrono; 


use super::monster_manager::{self, Monster};
use super::ui_manager; // Import ui_manager to switch back to monster list

/// A struct to hold the data for each individual combatant instance.
#[derive(Clone)]
struct Combatant {
    instance_name: String, // e.g., "Goblin 1"
    monster_template: Monster, // The base monster data
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
            
            let instance_name = if count > 1 || *current_count > 1 {
                format!("{} {}", monster.name, current_count)
            } else {
                monster.name.clone()
            };

            combatants.push(Combatant {
                instance_name,
                monster_template: monster.clone(),
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

    // Add Exit Simulation button
    let exit_button = Button::builder()
        .label("Exit Simulation")
        .halign(Align::Center) // Center the button horizontally
        .margin_bottom(12) // Add some margin below it
        .build();
    exit_button.add_css_class("destructive-action"); // A good class for exit/cancel buttons

    let app_clone = app.clone();
    let window_clone = window.clone();
    exit_button.connect_clicked(move |_| {
        // Return to the monster list view
        ui_manager::switch_to_monster_list(&app_clone, &window_clone);
    });
    main_vbox.append(&exit_button);

    // --- Console Output Section ---
    let console_scrolled_window = ScrolledWindow::builder()
        .height_request(150) // Fixed height for the console (changed from 75 to 150)
        .hexpand(true) // Expand horizontally
        .margin_bottom(12) // Margin between console and cards
        .build();

    let console_text_view = TextView::builder()
        .editable(false) // Make it read-only
        .wrap_mode(gtk::WrapMode::Word) // Wrap words
        .build();
    
    // An Rc<RefCell<TextBuffer>> to allow sharing and inner mutability of the buffer
    let console_buffer = Rc::new(RefCell::new(console_text_view.buffer()));
    console_scrolled_window.set_child(Some(&console_text_view));
    main_vbox.append(&console_scrolled_window);

    // Initial message to the console
    if let Ok(buffer) = console_buffer.try_borrow_mut() {
        buffer.insert(&mut buffer.start_iter(), "Simulation console: Last 50 lines will be displayed here.\n");
    }


    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true); // Ensure scrolled window expands within its parent
    scrolled_window.set_vexpand_set(true); // Allow explicit vertical expansion

    let flow_box = FlowBox::builder()
        .valign(Align::Start)
        .max_children_per_line(4)
        .min_children_per_line(1)
        .selection_mode(gtk::SelectionMode::None)
        .margin_top(12).margin_bottom(12).margin_start(12).margin_end(12)
        .build();

    for combatant in combatants {
        // Pass the console buffer to the card creation function
        let card = create_combatant_card(&combatant, Rc::clone(&console_buffer), console_text_view.clone());
        flow_box.insert(&card, -1);
    }

    scrolled_window.set_child(Some(&flow_box));
    main_vbox.append(&scrolled_window);


    window.set_content(Some(&main_vbox));
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
fn create_combatant_card(combatant: &Combatant, console_buffer: Rc<RefCell<TextBuffer>>, console_text_view: TextView) -> Frame {
    let card_frame = Frame::builder()
        .label(&combatant.instance_name)
        .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
        .build();
    
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
        .build();

    // --- HP and AC Box ---
    let stats_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    
    let hp_label = Label::new(Some("HP:"));
    let hp_adj = Adjustment::new(combatant.monster_template.hp as f64, 0.0, 9999.0, 1.0, 10.0, 0.0);
    let hp_spin_button = SpinButton::builder()
        .adjustment(&hp_adj)
        .numeric(true)
        .build();
    
    let ac_label = Label::new(Some(&format!("AC: {}", combatant.monster_template.ac)));

    stats_box.append(&hp_label);
    stats_box.append(&hp_spin_button);
    stats_box.append(&ac_label);
    vbox.append(&stats_box);

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

            let attack_details = format!(
                "• {} ({}{}, {}/turn)",
                attack.attack_name,
                attack.num_dice,
                attack.dice_used,
                attack.num_attacks,
            );
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
            let console_buffer_clone = Rc::clone(&console_buffer);
            let console_text_view_clone = console_text_view.clone();

            use_button.connect_clicked(move |_| {
                let creature_name = combatant_clone.instance_name.clone();
                let attack_name = attack_clone.attack_name.clone();
                let attacks_per_turn = attack_clone.num_attacks;

                if let Ok(buffer) = console_buffer_clone.try_borrow_mut() {
                    let mut iter = buffer.end_iter();
                    
                    buffer.insert(&mut iter, &format!("{}: {} started an attack using {} {} times.\n",
                        chrono::Local::now().format("%H:%M:%S"), creature_name, attack_name, attacks_per_turn));

                    for i in 0..attacks_per_turn {
                        // Calculate To Hit
                        let mut rng = rand::rngs::ThreadRng::default();
                        let d20_roll = rng.random_range(1..=20);
                        let ability_mod = match attack_clone.ability_used.as_str() {
                            "str" => combatant_clone.monster_template.str_mod,
                            "dex" => combatant_clone.monster_template.dex_mod,
                            "con" => combatant_clone.monster_template.con_mod,
                            "int" => combatant_clone.monster_template.int_mod,
                            "wis" => combatant_clone.monster_template.wis_mod,
                            "cha" => combatant_clone.monster_template.cha_mod,
                            _ => 0, // Should not happen if dropdown is constrained
                        };
                        let to_hit = d20_roll + ability_mod + combatant_clone.monster_template.pb;

                        // Calculate Damage using the new helper function
                        let (total_damage, damage_output) = calculate_damage(
                            attack_clone.num_dice,
                            &attack_clone.dice_used,
                            ability_mod,
                        );

                        buffer.insert(&mut iter, &format!("  Attack {}: To hit: {}; Damage: {}\n", i + 1, to_hit, damage_output));
                    }
                    
                    // Limit to last 50 lines
                    let mut full_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false).to_string();
                    let lines: Vec<&str> = full_text.lines().collect();
                    if lines.len() > 50 {
                        let start_index = lines.len() - 50;
                        let trimmed_text = lines[start_index..].join("\n");
                        buffer.set_text(&trimmed_text);
                    }

                    // Scroll to the end
                    let end_iter = buffer.end_iter();
                    let mark = buffer.create_mark(Some("end_mark"), &end_iter, false);
                    console_text_view_clone.scroll_to_mark(&mark, 0.0, false, 0.0, 0.0);
                }
            });
            attack_hbox.append(&use_button);
            vbox.append(&attack_hbox);
        }
    }

    card_frame.set_child(Some(&vbox));
    card_frame
}
