// simulation.rs

use gtk::{
    prelude::*,
    Align,
    Box,
    DropDown,
    FlowBox,
    Frame,
    Label,
    ListBox,
    Orientation,
    ScrolledWindow,
    SpinButton,
    StringObject,
    TextView,
};
use libadwaita::Application as AdwApplication;
use gtk::ApplicationWindow as AdwWindow;
use std::collections::HashMap;
use std::fs::{ self, File };
use std::io::{ self, Read, Write };
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use chrono;

use super::monster_manager::{ self, Monster, Attack, get_base_path };
use super::ui_factory::{ UiFactory };
use super::interface;

// =========================================================================
// Data Models & States
// =========================================================================

/// A struct to hold the data for each individual combatant instance.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Combatant {
    instance_name: String,
    monster_template: Monster,
    current_hp: i32,
    max_hp: i32,
}

/// A struct to hold the shared state of the simulation.
#[derive(Clone, Debug)]
pub struct SimulationState {
    combatants: Rc<RefCell<Vec<Combatant>>>,
    killed_monsters: Rc<RefCell<Vec<Combatant>>>,
    pub flow_box: FlowBox,
    pub console_buffer: Rc<RefCell<gtk::TextBuffer>>,
    pub console_text_view: gtk::TextView,
    pub roll_mode_dropdown: gtk::DropDown,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StaticSimulationState {
    combatants: Vec<Combatant>,
    killed_monsters: Vec<Combatant>,
}

impl StaticSimulationState {
    fn make_static(simulation_state: &SimulationState) -> Self {
        let combatants = simulation_state.combatants.borrow().clone();
        let killed_monsters = simulation_state.killed_monsters.borrow().clone();
        Self {
            combatants,
            killed_monsters,
        }
    }

    fn replace_with_static(self, simulation_state: &SimulationState) {
        let mut combatants = simulation_state.combatants.borrow_mut();
        let mut killed = simulation_state.killed_monsters.borrow_mut();

        for mon in self.combatants {
            combatants.push(mon);
        }
        for mon in self.killed_monsters {
            killed.push(mon);
        }
    }
}

// =========================================================================
// Simulation Setup & High-Level Views
// =========================================================================

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

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title = UiFactory::create_label("Select Combatants", Align::Center, false, &["title-3"]);
    main_vbox.append(&title);

    let scrolled_window = UiFactory::create_scrolled_window(true, true, None);
    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
    list_box.add_css_class("boxed-list");

    let all_monsters = monster_manager::read_all_monsters();
    let mut spin_buttons: Vec<(SpinButton, Monster)> = Vec::new();

    if all_monsters.is_empty() {
        list_box.append(&Label::new(Some("No monsters exist. Please create one first.")));
    } else {
        for monster in all_monsters {
            let row = UiFactory::create_box(Orientation::Horizontal, 6, (6, 6, 6, 6));
            let name_label = UiFactory::create_label(&monster.name, Align::Start, false, &[]);
            name_label.set_hexpand(true);

            let spin_button = UiFactory::create_spin_button(0.0, 100.0, 1.0, 0.0);
            row.append(&name_label);
            row.append(&spin_button);
            list_box.append(&row);

            spin_buttons.push((spin_button, monster));
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let start_button = UiFactory::create_button(
        "Start Simulation",
        Align::End,
        Some("suggested-action")
    );
    main_vbox.append(&start_button);

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

        window_clone.close();
        start_simulation_view(&app_clone, &parent_window_clone, selected_monsters);
    });

    window.set_child(Some(&main_vbox));
    window.present();
}

/// This function takes the selected monsters and builds the main card view.
pub fn start_simulation_view(
    app: &AdwApplication,
    window: &AdwWindow,
    selected_monsters: Vec<(Monster, i32)>
) {
    window.set_title(Some("Mass Combat Decider - Simulation"));

    // --- Generate the list of individual combatants ---
    let mut combatants: Vec<Combatant> = Vec::new();
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
                current_hp: monster.hp,
                monster_template: monster.clone(),
                max_hp: monster.hp,
            });
        }
    }

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    main_vbox.set_vexpand(true);
    main_vbox.set_hexpand(true);

    // --- Top Title & Round Tracker Row ---
    let top_row = UiFactory::create_box(Orientation::Horizontal, 24, (0, 12, 0, 0));
    top_row.set_halign(Align::Center);

    let simulation_title = UiFactory::create_label(
        "Live Simulation",
        Align::Center,
        false,
        &["title-1"]
    );
    let round_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
    round_box.set_valign(Align::Center);

    let round_label = Label::new(Some("Round:"));
    let round_spin_button = UiFactory::create_spin_button(1.0, 999.0, 1.0, 1.0);

    round_box.append(&round_label);
    round_box.append(&round_spin_button);
    top_row.append(&simulation_title);
    top_row.append(&round_box);
    main_vbox.append(&top_row);

    // --- Console Output Section ---
    let console_scrolled_window = UiFactory::create_scrolled_window(false, true, Some(150));
    console_scrolled_window.set_margin_bottom(12);

    let console_text_view = TextView::builder()
        .editable(false)
        .wrap_mode(gtk::WrapMode::Word)
        .build();

    let console_buffer = Rc::new(RefCell::new(console_text_view.buffer()));
    console_scrolled_window.set_child(Some(&console_text_view));
    main_vbox.append(&console_scrolled_window);

    if let Ok(buffer) = console_buffer.try_borrow_mut() {
        buffer.insert(
            &mut buffer.start_iter(),
            "Simulation console: Last 50 lines will be displayed here.\n"
        );
    }

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_vexpand_set(true);

    let flow_box = FlowBox::builder()
        .valign(Align::Start)
        .max_children_per_line(4)
        .min_children_per_line(1)
        .selection_mode(gtk::SelectionMode::None)
        .row_spacing(12)
        .column_spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let shared_state = Rc::new(RefCell::new(combatants));

    // --- Roll Mode DropDown Setup ---
    let mode_options = ["Natural", "Advantage", "Disadvantage"];
    let string_list = gtk::StringList::new(&mode_options);
    let roll_mode_dropdown = gtk::DropDown
        ::builder()
        .model(&string_list)
        .selected(0)
        .valign(Align::Center)
        .build();

    let simulation_state = SimulationState {
        combatants: Rc::clone(&shared_state),
        killed_monsters: Rc::new(RefCell::new(Vec::new())),
        flow_box: flow_box.clone(),
        console_buffer: Rc::clone(&console_buffer),
        console_text_view: console_text_view.clone(),
        roll_mode_dropdown: roll_mode_dropdown.clone(),
    };

    if check_for_simulation() {
        if let Some(static_sim) = get_simulation() {
            let _ = remove_simulation_file();
            static_sim.replace_with_static(&simulation_state);
        }
    }

    for combatant in shared_state.borrow().iter() {
        let card = create_combatant_card(combatant, &simulation_state);
        simulation_state.flow_box.insert(&card, -1);
    }

    // --- Bottom Layout: Split Button Action Bar ---
    let bottom_bar = UiFactory::create_box(Orientation::Horizontal, 0, (0, 12, 0, 0));
    bottom_bar.set_hexpand(true);

    let left_actions_box = UiFactory::create_box(Orientation::Horizontal, 8, (0, 0, 0, 0));
    left_actions_box.set_halign(Align::Start);
    left_actions_box.set_hexpand(true);

    let edit_button = UiFactory::create_button(
        "Edit Simulation",
        Align::Center,
        Some("edit-action")
    );
    let app_clone = app.clone();
    let window_clone_edit = window.clone();
    let simulation_state_clone = simulation_state.clone();
    edit_button.connect_clicked(move |_| {
        show_edit_simulation_menu(&app_clone, &window_clone_edit, simulation_state_clone.clone());
    });
    left_actions_box.append(&edit_button);

    let save_button = UiFactory::create_button(
        "Save Simulation",
        Align::Center,
        Some("suggested-action")
    );
    let app_clone_save = app.clone();
    let window_clone_save = window.clone();
    let simulation_state_clone = simulation_state.clone();
    save_button.connect_clicked(move |_| {
        let _ = save_simulation(&simulation_state_clone);
        interface::switch_to_monster_list(&app_clone_save, &window_clone_save);
    });
    left_actions_box.append(&save_button);

    let exit_button = UiFactory::create_button(
        "Exit Simulation",
        Align::Center,
        Some("destructive-action")
    );
    let app_clone_exit = app.clone();
    let window_clone_exit = window.clone();
    exit_button.connect_clicked(move |_| {
        interface::switch_to_monster_list(&app_clone_exit, &window_clone_exit);
    });
    left_actions_box.append(&exit_button);

    let right_actions_box = UiFactory::create_box(Orientation::Horizontal, 12, (0, 0, 0, 0));
    right_actions_box.set_halign(Align::End);

    let killed_button = UiFactory::create_button(
        "See Killed",
        Align::Center,
        Some("see-killed-action")
    );
    let app_clone = app.clone();
    let window_clone_killed = window.clone();
    let simulation_state_clone = simulation_state.clone();
    killed_button.connect_clicked(move |_| {
        show_killed_monsters_menu(&app_clone, &window_clone_killed, simulation_state_clone.clone());
    });

    right_actions_box.append(&roll_mode_dropdown);
    right_actions_box.append(&killed_button);

    bottom_bar.append(&left_actions_box);
    bottom_bar.append(&right_actions_box);

    main_vbox.append(&bottom_bar);
    scrolled_window.set_child(Some(&simulation_state.flow_box));
    main_vbox.append(&scrolled_window);
    window.set_child(Some(&main_vbox));
}

// =========================================================================
// Modal Menu Operations (Killed List & Live Editing)
// =========================================================================

fn show_killed_monsters_menu(
    app: &AdwApplication,
    parent_window: &AdwWindow,
    simulation_state: SimulationState
) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Killed Monsters")
        .transient_for(parent_window)
        .modal(true)
        .default_width(450)
        .default_height(500)
        .build();

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title = UiFactory::create_label(
        "Killed Monsters and Total XP",
        Align::Center,
        false,
        &["title-3"]
    );
    main_vbox.append(&title);

    let scrolled_window = UiFactory::create_scrolled_window(true, true, None);
    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
    list_box.add_css_class("boxed-list");

    let killed_monsters = simulation_state.killed_monsters.borrow();
    let mut total_xp = 0;

    if killed_monsters.is_empty() {
        list_box.append(&Label::new(Some("No monsters have been killed yet.")));
    } else {
        for combatant in killed_monsters.iter() {
            let row = UiFactory::create_box(Orientation::Horizontal, 6, (6, 6, 6, 6));
            let label_text = format!(
                "{} ({} XP)",
                combatant.instance_name,
                combatant.monster_template.exp
            );
            let name_label = UiFactory::create_label(&label_text, Align::Start, false, &[]);
            name_label.set_hexpand(true);

            row.append(&name_label);
            list_box.append(&row);
            total_xp += combatant.monster_template.exp;
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let xp_label = UiFactory::create_label(
        &format!("Total XP: {}", total_xp),
        Align::End,
        false,
        &["title-4", "suggested-action"]
    );
    main_vbox.append(&xp_label);

    let close_button = UiFactory::create_button("Close", Align::End, Some("destructive-action"));
    let window_clone = window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });
    main_vbox.append(&close_button);

    window.set_child(Some(&main_vbox));
    window.present();
}

pub fn show_edit_simulation_menu(
    app: &AdwApplication,
    parent_window: &AdwWindow,
    simulation_state: SimulationState
) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Edit Simulation")
        .transient_for(parent_window)
        .modal(true)
        .default_width(450)
        .default_height(500)
        .build();

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title = UiFactory::create_label("Edit Combatants", Align::Center, false, &["title-3"]);
    main_vbox.append(&title);

    let scrolled_window = UiFactory::create_scrolled_window(true, true, None);
    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
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
            let row = UiFactory::create_box(Orientation::Horizontal, 6, (6, 6, 6, 6));
            let name_label = UiFactory::create_label(&monster.name, Align::Start, false, &[]);
            name_label.set_hexpand(true);

            let initial_value = *initial_counts.get(&monster.name).unwrap_or(&0) as f64;
            let spin_button = UiFactory::create_spin_button(0.0, 100.0, 1.0, initial_value);

            row.append(&name_label);
            row.append(&spin_button);
            list_box.append(&row);
            spin_buttons.push((spin_button, monster));
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let start_button = UiFactory::create_button(
        "Update Simulation",
        Align::End,
        Some("suggested-action")
    );
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

    window.set_child(Some(&main_vbox));
    window.present();
}

fn update_simulation_view(
    selected_monsters: &Vec<(Monster, i32)>,
    simulation_state: &SimulationState
) {
    let mut current_combatants = simulation_state.combatants.borrow_mut();

    let mut existing_combatants_map: HashMap<String, Vec<Combatant>> = HashMap::new();
    for combatant in current_combatants.drain(..) {
        existing_combatants_map
            .entry(combatant.monster_template.name.clone())
            .or_insert_with(Vec::new)
            .push(combatant);
    }

    let mut new_combatant_list: Vec<Combatant> = Vec::new();

    for (monster_template, desired_count) in selected_monsters {
        let monster_name = &monster_template.name;

        let mut existing_of_type = existing_combatants_map
            .remove(monster_name)
            .unwrap_or_else(Vec::new);
        existing_of_type.sort_by(|a, b| a.instance_name.cmp(&b.instance_name));

        let num_existing = existing_of_type.len();
        let num_needed = *desired_count as usize;

        let mut existing_counter = 0;

        for _ in 0..num_needed {
            if existing_counter < num_existing {
                let combatant_to_keep = existing_of_type.remove(0);
                new_combatant_list.push(combatant_to_keep);
                existing_counter += 1;
            } else {
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
                let new_counter = max_number + 1;

                let instance_name = if *desired_count > 1 {
                    format!("{} {}", monster_name, new_counter)
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

    *current_combatants = new_combatant_list;

    while let Some(child) = simulation_state.flow_box.first_child() {
        simulation_state.flow_box.remove(&child);
    }

    for combatant in current_combatants.iter() {
        let card = create_combatant_card(combatant, simulation_state);
        simulation_state.flow_box.insert(&card, -1);
    }
}

// =========================================================================
// Dice Rolling & Mathematical Calculations
// =========================================================================

/// Core rolling logic helper
fn perform_d20_roll(mode: &str) -> (i32, Option<i32>) {
    let mut rng = rand::rngs::ThreadRng::default();
    let roll1 = rng.random_range(1..=20);
    let roll2 = rng.random_range(1..=20);

    match mode {
        "Advantage" => (std::cmp::max(roll1, roll2), Some(std::cmp::min(roll1, roll2))),
        "Disadvantage" => (std::cmp::min(roll1, roll2), Some(std::cmp::max(roll1, roll2))),
        _ => (roll1, None),
    }
}

fn calculate_damage(num_dice: i32, dice_used: &str, ability_mod: i32) -> (i32, String) {
    let mut rng = rand::rngs::ThreadRng::default();
    let dice_value_str = dice_used.trim_start_matches('d');
    let max_dice_val: i32 = dice_value_str.parse().unwrap_or(1);
    let mut total_damage = 0;
    let mut damage_rolls = Vec::new();

    for _ in 0..num_dice {
        let roll = rng.random_range(1..=max_dice_val);
        damage_rolls.push(roll.to_string());
        total_damage += roll;
    }

    let mut damage_output = String::new();
    if !damage_rolls.is_empty() {
        damage_output.push_str(&damage_rolls.join(" + "));
    }

    if ability_mod != 0 || damage_rolls.is_empty() {
        if !damage_rolls.is_empty() {
            damage_output.push_str(" + ");
        }
        damage_output.push_str(&ability_mod.to_string());
    }

    total_damage += ability_mod;
    damage_output.push_str(&format!(" = {}", total_damage));

    (total_damage, damage_output)
}

pub fn get_dropdown_text(dropdown: &DropDown) -> String {
    dropdown
        .selected_item()
        .and_then(|obj| {
            obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string())
        })
        .unwrap_or_default()
}

// =========================================================================
// Card Modular Parts Creation
// =========================================================================

/// Creates the Header row containing Name and the "Kill" action.
fn create_card_header(
    combatant: &Combatant,
    card_frame: &Frame,
    simulation_state: &SimulationState
) -> Box {
    let header_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));

    let name_label = UiFactory::create_label(
        &combatant.instance_name,
        Align::Start,
        false,
        &["title-4"]
    );
    name_label.set_hexpand(true);

    let kill_button = UiFactory::create_button("Kill", Align::End, Some("destructive-action"));

    let card_frame_clone = card_frame.clone();
    let combatants_clone = Rc::clone(&simulation_state.combatants);
    let killed_monsters_clone = Rc::clone(&simulation_state.killed_monsters);
    let combatant_instance_name = combatant.instance_name.clone();
    let flow_box_clone = simulation_state.flow_box.clone();

    kill_button.connect_clicked(move |_| {
        if let Ok(mut combatants) = combatants_clone.try_borrow_mut() {
            if
                let Some(pos) = combatants
                    .iter()
                    .position(|c| c.instance_name == combatant_instance_name)
            {
                let killed = combatants.remove(pos);
                killed_monsters_clone.borrow_mut().push(killed);
            }
        }
        flow_box_clone.remove(&card_frame_clone);
    });

    header_box.append(&name_label);
    header_box.append(&kill_button);
    header_box
}

/// Creates the HP, AC, and Speed control panel
fn create_stats_row(
    combatant: &Combatant,
    card_frame: &Frame,
    simulation_state: &SimulationState
) -> Box {
    let stats_box = UiFactory::create_box(Orientation::Horizontal, 12, (0, 0, 0, 0));

    let hp_label = Label::new(Some("HP:"));
    let hp_spin_button = UiFactory::create_spin_button(
        0.0,
        combatant.max_hp.into(),
        1.0,
        combatant.current_hp as f64
    );
    let max_hp_label = Label::new(Some(&format!("Max HP: {}", combatant.max_hp)));

    if combatant.current_hp <= combatant.max_hp / 2 {
        card_frame.add_css_class("bloodied");
    }

    let combatants_clone = Rc::clone(&simulation_state.combatants);
    let combatant_instance_name_clone = combatant.instance_name.clone();
    let card_frame_clone = card_frame.clone();
    let max_hp = combatant.max_hp;

    hp_spin_button.connect_value_changed(move |btn| {
        let current_hp = btn.value() as i32;

        if let Ok(mut combatants) = combatants_clone.try_borrow_mut() {
            if
                let Some(c) = combatants
                    .iter_mut()
                    .find(|c| c.instance_name == combatant_instance_name_clone)
            {
                c.current_hp = current_hp;
            }
        }

        if current_hp <= max_hp / 2 {
            card_frame_clone.add_css_class("bloodied");
        } else {
            card_frame_clone.remove_css_class("bloodied");
        }
    });

    let ac_label = Label::new(Some(&format!("AC: {}", combatant.monster_template.ac)));
    let speed_label = Label::new(Some(&format!("Speed: {}", combatant.monster_template.speed)));

    stats_box.append(&hp_label);
    stats_box.append(&hp_spin_button);
    stats_box.append(&max_hp_label);
    stats_box.append(&ac_label);
    stats_box.append(&speed_label);
    stats_box
}

/// Creates Damage Vulnerability indicator if present
fn create_vulnerabilities_label(combatant: &Combatant) -> Option<Label> {
    if combatant.monster_template.vulnerabilities.is_empty() {
        return None;
    }

    let mut label_text = String::from("<b>Damage Vulnerabilities:</b>");
    for i in &combatant.monster_template.vulnerabilities {
        label_text.push(' ');
        label_text.push_str(i);
        label_text.push(',');
    }
    label_text.pop(); // Remove trailing comma

    let vuln_label = UiFactory::create_label(&label_text, Align::Start, true, &[]);
    vuln_label.set_margin_top(6);
    Some(vuln_label)
}

/// Creates Abilities info section
fn create_abilities_label(combatant: &Combatant) -> Label {
    let abilities_text = UiFactory::create_label(
        &format!("<b>Abilities</b>:\n{}", combatant.monster_template.abilities),
        Align::Start,
        true,
        &[]
    );
    abilities_text.set_wrap(true);
    abilities_text
}

/// Helper to scroll the simulation text window to the bottom.
fn scroll_console_to_bottom(text_view: &gtk::TextView) {
    if let Some(adj) = text_view.vadjustment() {
        adj.set_value(adj.upper());
    }
}

/// Helper to prune console output to preserve layout size
fn limit_console_buffer(buffer: &gtk::TextBuffer) {
    let line_count = buffer.line_count();
    if line_count > 50 {
        let lines_to_remove = line_count - 50;
        let mut start_iter = buffer.start_iter();
        start_iter.forward_lines(lines_to_remove);
        buffer.delete(&mut buffer.start_iter(), &mut start_iter);
    }
}

/// Creates the Saving Throw button dashboard
fn create_saves_grid(combatant: &Combatant, simulation_state: &SimulationState) -> Box {
    let container = UiFactory::create_box(Orientation::Vertical, 4, (0, 0, 0, 0));
    let label = UiFactory::create_label("<b>Saves</b>", Align::Start, true, &[]);
    label.set_margin_top(6);
    container.append(&label);

    let saves_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
    let stats = ["Str", "Dex", "Con", "Int", "Wis", "Cha"];

    for stat_name in stats.iter() {
        let stat_vbox = UiFactory::create_box(Orientation::Vertical, 3, (0, 0, 0, 0));
        let save_button = UiFactory::create_button(*stat_name, Align::Center, None);

        let combatant_clone = combatant.clone();
        let console_buffer_clone = Rc::clone(&simulation_state.console_buffer);
        let console_text_view_clone = simulation_state.console_text_view.clone();
        let stat_name_clone = stat_name.to_string();
        let mon = combatant_clone.monster_template.clone();

        let modifier = match stat_name_clone.to_lowercase().as_str() {
            "str" => mon.mods[0],
            "dex" => mon.mods[1],
            "con" => mon.mods[2],
            "int" => mon.mods[3],
            "wis" => mon.mods[4],
            "cha" => mon.mods[5],
            _ => 0,
        };

        let save_mod_label = UiFactory::create_label(
            &format!("{}: {}", stat_name_clone, modifier),
            Align::Center,
            false,
            &[]
        );
        let save_simulation_state_clone = simulation_state.clone();

        save_button.connect_clicked(move |_| {
            let mode = get_dropdown_text(&save_simulation_state_clone.roll_mode_dropdown);
            let (d20_roll, lost_roll) = perform_d20_roll(&mode);

            let save_bonus = match stat_name_clone.to_lowercase().as_str() {
                "str" => mon.pb * (mon.saves[0] as i32),
                "dex" => mon.pb * (mon.saves[1] as i32),
                "con" => mon.pb * (mon.saves[2] as i32),
                "int" => mon.pb * (mon.saves[3] as i32),
                "wis" => mon.pb * (mon.saves[4] as i32),
                "cha" => mon.pb * (mon.saves[5] as i32),
                _ => 0,
            };

            let total = d20_roll + modifier + save_bonus;

            if let Ok(buffer) = console_buffer_clone.try_borrow_mut() {
                let prefix = format!(
                    "{}: {} rolled a {} Save: (",
                    chrono::Local::now().format("%H:%M:%S"),
                    combatant_clone.instance_name,
                    stat_name_clone
                );
                let suffix = format!(
                    ") + {} (Mod) + {} (prof) = {}\n",
                    modifier,
                    save_bonus,
                    total
                );

                append_roll_to_console(&buffer, &prefix, d20_roll, lost_roll, &suffix);
                limit_console_buffer(&buffer);
            }
            scroll_console_to_bottom(&console_text_view_clone);
        });

        stat_vbox.append(&save_mod_label);
        stat_vbox.append(&save_button);
        saves_box.append(&stat_vbox);
    }

    container.append(&saves_box);
    container
}

/// Creates the Interactive Attacks List for the monster
fn create_attacks_list(combatant: &Combatant, simulation_state: &SimulationState) -> Option<Box> {
    if combatant.monster_template.attacks.is_empty() {
        return None;
    }

    let container = UiFactory::create_box(Orientation::Vertical, 4, (0, 0, 0, 0));
    let header_label = UiFactory::create_label("<b>Attacks</b>", Align::Start, true, &[]);
    header_label.set_margin_top(6);
    container.append(&header_label);

    for attack in &combatant.monster_template.attacks {
        let attack_hbox = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
        let save_dc = 8 + get_ability_mod(combatant, attack) + combatant.monster_template.pb;

        let attack_details = if !attack.saving_throw {
            format!(
                "• {} ({}{}, {}/turn)",
                attack.attack_name,
                attack.num_dice,
                attack.dice_used,
                attack.num_attacks
            )
        } else {
            format!(
                "• {} ({}{}, DC {})",
                attack.attack_name,
                attack.num_dice,
                attack.dice_used,
                save_dc
            )
        };

        let attack_label = UiFactory::create_label(&attack_details, Align::Start, false, &[]);
        attack_label.set_hexpand(true);
        attack_hbox.append(&attack_label);

        let use_button = UiFactory::create_button("Use", Align::Center, None);

        let combatant_clone = combatant.clone();
        let attack_clone = attack.clone();
        let console_buffer_clone = Rc::clone(&simulation_state.console_buffer);
        let console_text_view_clone = simulation_state.console_text_view.clone();
        let attack_simulation_state_clone = simulation_state.clone();

        use_button.connect_clicked(move |_| {
            let creature_name = combatant_clone.instance_name.clone();
            let attack_name = attack_clone.attack_name.clone();
            let attacks_per_turn = attack_clone.num_attacks;

            if let Ok(buffer) = console_buffer_clone.try_borrow_mut() {
                let mut iter = buffer.end_iter();

                if !attack_clone.saving_throw {
                    buffer.insert(
                        &mut iter,
                        &format!(
                            "{}: {} started an attack using {} {} times.\n",
                            chrono::Local::now().format("%H:%M:%S"),
                            creature_name,
                            attack_name,
                            attacks_per_turn
                        )
                    );

                    for i in 0..attacks_per_turn {
                        let mode = get_dropdown_text(
                            &attack_simulation_state_clone.roll_mode_dropdown
                        );
                        let (d20_roll, lost_roll) = perform_d20_roll(&mode);

                        let ability_mod = get_ability_mod(&combatant_clone, &attack_clone);
                        let total_mod = ability_mod + combatant_clone.monster_template.pb;
                        let to_hit = d20_roll + total_mod;
                        let crit_message = if d20_roll == 20 { " -> CRITICAL HIT!" } else { "" };

                        let (_, damage_output) = calculate_damage(
                            if d20_roll == 20 {
                                attack_clone.num_dice * 2
                            } else {
                                attack_clone.num_dice
                            },
                            &attack_clone.dice_used,
                            ability_mod
                        );

                        let prefix = format!("  Attack {}: To hit: (", i + 1);
                        let suffix = format!(
                            ") + {} (Total Mod) = {}{}; Damage: {}\n",
                            total_mod,
                            to_hit,
                            crit_message,
                            damage_output
                        );

                        append_roll_to_console(&buffer, &prefix, d20_roll, lost_roll, &suffix);
                    }
                } else {
                    buffer.insert(
                        &mut iter,
                        &format!(
                            "{}: {} started an attack using {}",
                            chrono::Local::now().format("%H:%M:%S"),
                            creature_name,
                            attack_name
                        )
                    );

                    let (_, damage_output) = calculate_damage(
                        attack_clone.num_dice,
                        &attack_clone.dice_used,
                        0
                    );
                    buffer.insert(&mut iter, &format!("  Damage: {}\n", damage_output));
                }

                limit_console_buffer(&buffer);
            }

            scroll_console_to_bottom(&console_text_view_clone);
        });

        attack_hbox.append(&use_button);
        container.append(&attack_hbox);
    }

    Some(container)
}

// =========================================================================
// Central Assembler Function
// =========================================================================

/// Helper function to create a single monster card for the simulation view.
fn create_combatant_card(combatant: &Combatant, simulation_state: &SimulationState) -> Frame {
    let card_frame = Frame::builder()
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();

    let vbox = UiFactory::create_box(Orientation::Vertical, 6, (6, 6, 6, 6));

    // Append 1: Header Row
    let header_box = create_card_header(combatant, &card_frame, simulation_state);
    vbox.append(&header_box);

    // Append 2: Statistics Panel (HP, AC, Speed)
    let stats_box = create_stats_row(combatant, &card_frame, simulation_state);
    vbox.append(&stats_box);

    // Append 3: Vulnerabilities (Optional)
    if let Some(vuln_label) = create_vulnerabilities_label(combatant) {
        vbox.append(&vuln_label);
    }

    // Append 4: Abilities Text Block
    let abilities_text = create_abilities_label(combatant);
    vbox.append(&abilities_text);

    // Append 5: Saves Controls
    let saves_control_panel = create_saves_grid(combatant, simulation_state);
    vbox.append(&saves_control_panel);

    // Append 6: Attacks Controls (Optional)
    if let Some(attacks_list) = create_attacks_list(combatant, simulation_state) {
        vbox.append(&attacks_list);
    }

    card_frame.set_child(Some(&vbox));
    card_frame
}

// =========================================================================
// Storage / Serialization Systems
// =========================================================================

fn save_simulation(simulation_state: &SimulationState) -> io::Result<()> {
    let mut path = get_base_path()?;
    if !path.exists() {
        fs::create_dir(&path)?;
    }

    path.push("active_simulation.json");
    let static_sim = StaticSimulationState::make_static(simulation_state);
    let json_data = serde_json::to_string_pretty(&static_sim)?;
    let mut file = File::create(&path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

fn get_simulation() -> Option<StaticSimulationState> {
    let mut path = get_base_path().ok()?;
    if !path.exists() {
        return None;
    }
    path.push("active_simulation.json");

    let mut file = File::open(&path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    match serde_json::from_str(&contents) {
        Ok(simulation) => Some(simulation),
        Err(e) => {
            eprintln!("Failed to parse monster JSON for active_simulation.json: {}", e);
            None
        }
    }
}

pub fn remove_simulation_file() -> io::Result<()> {
    if check_for_simulation() {
        let mut path = get_base_path()?;
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        path.push("active_simulation.json");
        fs::remove_file(&path)?;
    }
    Ok(())
}

pub fn check_for_simulation() -> bool {
    if let Ok(mut path) = get_base_path() {
        path.push("active_simulation.json");
        return path.exists();
    }
    false
}

fn get_ability_mod(combatant: &Combatant, attack: &Attack) -> i32 {
    match attack.ability_used.as_str() {
        "str" => combatant.monster_template.mods[0],
        "dex" => combatant.monster_template.mods[1],
        "con" => combatant.monster_template.mods[2],
        "int" => combatant.monster_template.mods[3],
        "wis" => combatant.monster_template.mods[4],
        "cha" => combatant.monster_template.mods[5],
        _ => 0,
    }
}

fn append_roll_to_console(
    buffer: &gtk::TextBuffer,
    prefix: &str,
    won_roll: i32,
    lost_roll: Option<i32>,
    suffix: &str
) {
    let tag_table = buffer.tag_table();
    if tag_table.lookup("strikethrough").is_none() {
        let tag = gtk::TextTag::builder().name("strikethrough").strikethrough(true).build();
        tag_table.add(&tag);
    }

    let mut iter = buffer.end_iter();
    buffer.insert(&mut iter, prefix);

    if let Some(lost) = lost_roll {
        buffer.insert(&mut iter, &format!("{} | ", won_roll));
        let start_offset = buffer.char_count();
        buffer.insert(&mut iter, &lost.to_string());
        let end_offset = buffer.char_count();

        let start_strike = buffer.iter_at_offset(start_offset);
        let end_strike = buffer.iter_at_offset(end_offset);
        buffer.apply_tag_by_name("strikethrough", &start_strike, &end_strike);
    } else {
        buffer.insert(&mut iter, &won_roll.to_string());
    }

    let mut end_iter = buffer.end_iter();
    buffer.insert(&mut end_iter, suffix);
}
