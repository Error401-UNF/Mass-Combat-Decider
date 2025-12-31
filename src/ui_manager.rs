// uimanager.rs
//
// This file manages the user interface for the Mass Combat Decider application.
// It uses GTK4 and Libadwaita to create windows, forms, and lists for managing monsters.

use gtk::{
    prelude::*, DropDown, Entry, Label, ListBox, Orientation, ScrolledWindow, StringList,
    StringObject, pango, Grid,
};
use gtk::{Button, Align, Box};
use libadwaita::{prelude::*,};
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;

use crate::monster_manager::Monster;

use super::{monster_manager, simulation};

// Displays a modal window for creating a new monster.
pub fn show_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow) {
    // Create the modal window with a title and dimensions.
    let window = AdwWindow::builder()
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(500)
        .default_height(500)
        .modal(true)
        .build();

    // Create the main vertical box to hold all UI elements.
    let window_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    // Title label for the menu.
    let title_label = Label::builder()
        .label("Create a Monster")
        .halign(Align::Center)
        .build();
    title_label.add_css_class("title-1");

    // Grid for arranging the input fields. Grids are ideal for form-like layouts.
    let input_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();

    // Create entry fields for monster stats.
    let (monster_name_label, monster_name_entry) = create_label_entry_pair("Monster Name:", "Enter monster name...");
    let (hp_label, hp_entry) = create_label_entry_pair("HP:", "Enter hit points...");
    let (ac_label, ac_entry) = create_label_entry_pair("AC:", "Enter armor class...");
    let (exp_label, exp_entry) = create_label_entry_pair("EXP:", "Enter experience points...");

    // Create dropdowns for stats with predefined values.
    let (prof_label, prof_entry) = create_label_entry_pair("Proficiency Bonus:", "1");
    let (str_label, str_entry) = create_label_entry_pair("Strength Mod:", "0");
    let (dex_label, dex_entry) = create_label_entry_pair("Dexterity Mod:", "0");
    let (con_label, con_entry) = create_label_entry_pair("Constitution Mod:", "0");
    let (int_label, int_entry) = create_label_entry_pair("Intelligence Mod:", "0");
    let (wis_label, wis_entry) = create_label_entry_pair("Wisdom Mod:", "0");
    let (cha_label, cha_entry) = create_label_entry_pair("Charisma Mod:", "0");

    // Attach widgets to the grid.
    input_grid.attach(&monster_name_label, 0, 0, 1, 1);
    input_grid.attach_next_to(&monster_name_entry, Some(&monster_name_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&hp_label, 0, 1, 1, 1);
    input_grid.attach_next_to(&hp_entry, Some(&hp_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&ac_label, 0, 2, 1, 1);
    input_grid.attach_next_to(&ac_entry, Some(&ac_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&exp_label, 0, 3, 1, 1);
    input_grid.attach_next_to(&exp_entry, Some(&exp_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&prof_label, 0, 4, 1, 1);
    input_grid.attach_next_to(&prof_entry, Some(&prof_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&str_label, 0, 5, 1, 1);
    input_grid.attach_next_to(&str_entry, Some(&str_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&dex_label, 0, 6, 1, 1);
    input_grid.attach_next_to(&dex_entry, Some(&dex_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&con_label, 0, 7, 1, 1);
    input_grid.attach_next_to(&con_entry, Some(&con_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&int_label, 0, 8, 1, 1);
    input_grid.attach_next_to(&int_entry, Some(&int_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&wis_label, 0, 9, 1, 1);
    input_grid.attach_next_to(&wis_entry, Some(&wis_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&cha_label, 0, 10, 1, 1);
    input_grid.attach_next_to(&cha_entry, Some(&cha_label), gtk::PositionType::Right, 1, 1);

    // Label for displaying validation errors to the user.
    let error_label = Label::builder()
        .label("")
        .halign(Align::Center)
        .margin_top(6)
        .margin_bottom(6)
        .build();

    // Create buttons for saving or canceling.
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::End)
        .build();

    let save_button = Button::builder()
        .label("Save Monster")
        .build();

    let cancel_button = Button::builder()
        .label("Cancel")
        .build();

    button_box.append(&save_button);
    button_box.append(&cancel_button);

    // Clone widgets for use in closures.
    let monster_name_entry_clone = monster_name_entry.clone();
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let exp_entry_clone = exp_entry.clone();
    let prof_entry_clone = prof_entry.clone();
    let str_entry_clone = str_entry.clone();
    let dex_entry_clone = dex_entry.clone();
    let con_entry_clone = con_entry.clone();
    let int_entry_clone = int_entry.clone();
    let wis_entry_clone = wis_entry.clone();
    let cha_entry_clone = cha_entry.clone();
    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    let error_label_clone = error_label.clone();

    // Connect save button to the logic for creating and saving the monster.
    save_button.connect_clicked(move |_| {
        error_label_clone.set_text(""); // Clear previous errors.

        // Helper function to safely parse integer values from Entry widgets.
        let parse_int_entry = |entry: &Entry, field_name: &str| -> Result<i32, String> {
            entry.text()
                .parse::<i32>()
                .map_err(|_| format!("'{}' must be a valid number.", field_name))
        };
        
        let name = monster_name_entry_clone.text().to_string();
        if name.trim().is_empty() {
            error_label_clone.set_text("Monster name cannot be empty.");
            return;
        }

        let hp = match parse_int_entry(&hp_entry_clone, "HP") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let ac = match parse_int_entry(&ac_entry_clone, "AC") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };
        
        let exp = match parse_int_entry(&exp_entry_clone, "EXP") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let pb = match parse_int_entry(&prof_entry_clone, "Proficiency Bonus") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let str_mod = match parse_int_entry(&str_entry_clone, "Strength Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let dex_mod = match parse_int_entry(&dex_entry_clone, "Dexterity Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let con_mod = match parse_int_entry(&con_entry_clone, "Constitution Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let int_mod = match parse_int_entry(&int_entry_clone, "Intelligence Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let wis_mod = match parse_int_entry(&wis_entry_clone, "Wisdom Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let cha_mod = match parse_int_entry(&cha_entry_clone, "Charisma Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        // Create the new Monster struct.
        let new_monster = monster_manager::Monster {
            name,
            hp,
            ac,
            exp,
            pb,
            str_mod,
            dex_mod,
            con_mod,
            int_mod,
            wis_mod,
            cha_mod,
            attacks: Vec::new(),
        };

        // Save the monster and handle potential errors.
        if let Err(e) = monster_manager::save_monster(new_monster) {
            error_label_clone.set_text(&format!("Failed to save monster: {}", e));
            return;
        }

        // Close the modal and refresh the parent window's monster list.
        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    // Connect cancel button to simply close the window.
    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    // Assemble the main window layout.
    window_vbox.append(&title_label);
    window_vbox.append(&input_grid);
    window_vbox.append(&error_label);
    window_vbox.append(&button_box);

    window.set_content(Some(&window_vbox));
    window.present();
}

pub fn edit_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow, monster: Monster) {
    // Create the modal window with a title and dimensions.
    let window = AdwWindow::builder()
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(500)
        .default_height(500)
        .modal(true)
        .build();

    // Create the main vertical box to hold all UI elements.
    let window_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    // Title label for the menu.
    let title_label = Label::builder()
        .label(format!("Modify {}",monster.name))
        .halign(Align::Center)
        .build();
    title_label.add_css_class("title-1");

    // Grid for arranging the input fields. Grids are ideal for form-like layouts.
    let input_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();

    // Create entry fields for monster stats.
    let (hp_label, hp_entry) = create_text_label_entry_pair("HP:", &format!("{}",monster.hp));
    let (ac_label, ac_entry) = create_text_label_entry_pair("AC:", &format!("{}",monster.ac));
    let (exp_label, exp_entry) = create_text_label_entry_pair("EXP:", &format!("{}",monster.exp));

    // Create dropdowns for stats with predefined values.
    let (prof_label, prof_entry) = create_text_label_entry_pair("Proficiency Bonus:", &format!("{}",monster.pb));
    let (str_label, str_entry) = create_text_label_entry_pair("Strength Mod:", &format!("{}",monster.str_mod));
    let (dex_label, dex_entry) = create_text_label_entry_pair("Dexterity Mod:", &format!("{}",monster.dex_mod));
    let (con_label, con_entry) = create_text_label_entry_pair("Constitution Mod:", &format!("{}",monster.con_mod));
    let (int_label, int_entry) = create_text_label_entry_pair("Intelligence Mod:", &format!("{}",monster.int_mod));
    let (wis_label, wis_entry) = create_text_label_entry_pair("Wisdom Mod:", &format!("{}",monster.wis_mod));
    let (cha_label, cha_entry) = create_text_label_entry_pair("Charisma Mod:", &format!("{}",monster.cha_mod));

    // Attach widgets to the grid.
    input_grid.attach(&hp_label, 0, 1, 1, 1);
    input_grid.attach_next_to(&hp_entry, Some(&hp_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&ac_label, 0, 2, 1, 1);
    input_grid.attach_next_to(&ac_entry, Some(&ac_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&exp_label, 0, 3, 1, 1);
    input_grid.attach_next_to(&exp_entry, Some(&exp_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&prof_label, 0, 4, 1, 1);
    input_grid.attach_next_to(&prof_entry, Some(&prof_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&str_label, 0, 5, 1, 1);
    input_grid.attach_next_to(&str_entry, Some(&str_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&dex_label, 0, 6, 1, 1);
    input_grid.attach_next_to(&dex_entry, Some(&dex_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&con_label, 0, 7, 1, 1);
    input_grid.attach_next_to(&con_entry, Some(&con_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&int_label, 0, 8, 1, 1);
    input_grid.attach_next_to(&int_entry, Some(&int_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&wis_label, 0, 9, 1, 1);
    input_grid.attach_next_to(&wis_entry, Some(&wis_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&cha_label, 0, 10, 1, 1);
    input_grid.attach_next_to(&cha_entry, Some(&cha_label), gtk::PositionType::Right, 1, 1);

    // Label for displaying validation errors to the user.
    let error_label = Label::builder()
        .label("")
        .halign(Align::Center)
        .margin_top(6)
        .margin_bottom(6)
        .build();

    // Create buttons for saving or canceling.
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::End)
        .build();

    let save_button = Button::builder()
        .label("Save Monster")
        .build();

    let cancel_button = Button::builder()
        .label("Cancel")
        .build();

    button_box.append(&save_button);
    button_box.append(&cancel_button);

    // Clone widgets for use in closures.
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let exp_entry_clone = exp_entry.clone();
    let prof_entry_clone = prof_entry.clone();
    let str_entry_clone = str_entry.clone();
    let dex_entry_clone = dex_entry.clone();
    let con_entry_clone = con_entry.clone();
    let int_entry_clone = int_entry.clone();
    let wis_entry_clone = wis_entry.clone();
    let cha_entry_clone = cha_entry.clone();
    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    let error_label_clone = error_label.clone();

    // Connect save button to the logic for creating and saving the monster.
    save_button.connect_clicked(move |_| {
        error_label_clone.set_text(""); // Clear previous errors.

        // Helper function to safely parse integer values from Entry widgets.
        let parse_int_entry = |entry: &Entry, field_name: &str| -> Result<i32, String> {
            entry.text()
                .parse::<i32>()
                .map_err(|_| format!("'{}' must be a valid number.", field_name))
        };
        
        let hp = match parse_int_entry(&hp_entry_clone, "HP") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let ac = match parse_int_entry(&ac_entry_clone, "AC") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };
        
        let exp = match parse_int_entry(&exp_entry_clone, "EXP") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let pb = match parse_int_entry(&prof_entry_clone, "Proficiency Bonus") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let str_mod = match parse_int_entry(&str_entry_clone, "Strength Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let dex_mod = match parse_int_entry(&dex_entry_clone, "Dexterity Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let con_mod = match parse_int_entry(&con_entry_clone, "Constitution Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let int_mod = match parse_int_entry(&int_entry_clone, "Intelligence Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let wis_mod = match parse_int_entry(&wis_entry_clone, "Wisdom Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        let cha_mod = match parse_int_entry(&cha_entry_clone, "Charisma Mod") {
            Ok(val) => val,
            Err(e) => {
                error_label_clone.set_text(&e);
                return;
            }
        };

        // Create the new Monster struct.
        let new_monster = monster_manager::Monster {
            name: monster.name.clone(),
            hp,
            ac,
            exp,
            pb,
            str_mod,
            dex_mod,
            con_mod,
            int_mod,
            wis_mod,
            cha_mod,
            attacks: monster.attacks.clone(),
        };

        // Save the monster and handle potential errors.
        if let Err(e) = monster_manager::save_monster(new_monster) {
            error_label_clone.set_text(&format!("Failed to save monster: {}", e));
            return;
        }

        // Close the modal and refresh the parent window's monster list.
        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    // Connect cancel button to simply close the window.
    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    // Assemble the main window layout.
    window_vbox.append(&title_label);
    window_vbox.append(&input_grid);
    window_vbox.append(&error_label);
    window_vbox.append(&button_box);

    window.set_content(Some(&window_vbox));
    window.present();
}

// Switches the main window's content to the "first time" welcome view.
pub fn switch_to_first_time(app: &AdwApplication, window: &AdwWindow) {
    let welcome = Label::builder()
        .label(
            "Welcome to the Mass Combat Decider.\nThis app is designed to assist you in making combat easier and faster for the DM. It's meant to provide quick calculations for things like # of attacks, attack, and damage rolls.\n\nSince you don't have any monsters yet, create your first one using the button below!"
        )
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .wrap(true)
        .halign(Align::Center)
        .build();

    let create_monster_button = Button::builder()
        .label("Create Monster")
        .margin_top(12)
        .halign(Align::Center)
        .build();
    create_monster_button.add_css_class("suggested-action");

    // Clone app and window to move into the button's closure.
    let app_clone = app.clone();
    let window_clone = window.clone();
    create_monster_button.connect_clicked(move |_| {
        show_monster_creation_menu(&app_clone, &window_clone);
    });

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .halign(Align::Center)
        .build();

    vbox.append(&welcome);
    vbox.append(&create_monster_button);

    window.set_content(Some(&vbox));
    window.present();
}

// Switches the main window's content to the list of all created monsters.
pub fn switch_to_monster_list(app: &AdwApplication, window: &AdwWindow) {
    window.set_title(Some("Mass Combat Decider - Your Monsters"));

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    let title_label = Label::builder()
        .label("Your Monsters")
        .halign(Align::Center)
        .margin_bottom(12)
        .build();
    title_label.add_css_class("title-1");

    // Box for the top action buttons.
    let top_button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Center)
        .margin_bottom(12)
        .build();

    let create_monster_button = Button::builder()
        .label("Create New Monster")
        .build();
    
    let start_simulation_button = Button::builder()
        .label("Start Simulation")
        .build();
    start_simulation_button.add_css_class("suggested-action");

    let continue_simulation_button = Button::builder()
        .label("Continue Simulation")
        .build();

    top_button_box.append(&create_monster_button);
    top_button_box.append(&start_simulation_button);
    if simulation::check_for_simulation(){
        top_button_box.append(&continue_simulation_button);
    }
    
    main_vbox.append(&title_label);
    main_vbox.append(&top_button_box);

    let app_clone = app.clone();
    let window_clone = window.clone();
    create_monster_button.connect_clicked(move |_| {
        show_monster_creation_menu(&app_clone, &window_clone);
    });

    let app_clone_sim = app.clone();
    let window_clone_sim = window.clone();
    start_simulation_button.connect_clicked(move |_| {
        let _ = simulation::remove_simulation_file();
        simulation::show_simulation_setup_menu(&app_clone_sim, &window_clone_sim);
    });

    let app_clone_continue_sim = app.clone();
    let window_clone_continue_sim = window.clone();
    continue_simulation_button.connect_clicked(move |_| {
        simulation::start_simulation_view(&app_clone_continue_sim, &window_clone_continue_sim, Vec::new());
    });
    
    // Scrolled window to handle a long list of monsters.
    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");

    // Read all monsters from the JSON file.
    let monsters = monster_manager::read_all_monsters();

    if monsters.is_empty() {
        let no_monsters_label = Label::builder()
            .label("No monsters found. Click 'Create New Monster' to add one.")
            .halign(Align::Center)
            .valign(Align::Center)
            .vexpand(true)
            .hexpand(true)
            .build();
        list_box.append(&no_monsters_label);
    } else {
        for monster in monsters {
            // Create a custom row for each monster.
            let row = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .margin_top(6)
                .margin_bottom(6)
                .margin_start(12)
                .margin_end(12)
                .build();
            
            // Left side: Name and stats.
            let info_vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .halign(Align::Start)
                .hexpand(true)
                .spacing(3)
                .build();

            let name_label = Label::builder()
                .label(&format!("<b>{}</b>", monster.name))
                .use_markup(true)
                .halign(Align::Start)
                .build();

            let stats_label = Label::builder()
                .label(&format!("HP: {}, AC: {}, EXP: {}, PB: {}, \nSTR: {}, DEX: {}, CON: {}, INT: {}, WIS: {}, CHA: {}",
                    monster.hp, monster.ac, monster.exp, monster.pb,
                    monster.str_mod, monster.dex_mod, monster.con_mod,
                    monster.int_mod, monster.wis_mod, monster.cha_mod))
                .halign(Align::Start)
                .build();

            let attacks_str = monster.attacks.iter()
                .map(|a| a.attack_name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            
            let attacks_label = Label::builder()
                .label(&format!("Attacks: {}", if attacks_str.is_empty() { "None" } else { &attacks_str }))
                .halign(Align::Start)
                .ellipsize(pango::EllipsizeMode::End)
                .tooltip_text(&attacks_str)
                .build();

            info_vbox.append(&name_label);
            info_vbox.append(&stats_label);
            info_vbox.append(&attacks_label);

            // Right side: Action buttons.
            let button_box = Box::builder()
                .orientation(Orientation::Horizontal)
                .halign(Align::End)
                .spacing(6)
                .build();

            let edit_monster_button = Button::with_label("Edit");
            let add_attack_button = Button::with_label("Add Attack");
            let remove_attack_button = Button::with_label("Remove Attack");
            let delete_button = Button::with_label("Delete");
            delete_button.add_css_class("destructive-action");
            
            button_box.append(&edit_monster_button);
            button_box.append(&add_attack_button);
            button_box.append(&remove_attack_button);
            button_box.append(&delete_button);

            // Connect button signals.
            let monster_for_edit = monster.clone();
            let app_clone_for_edit = app.clone();
            let window_clone_for_edit = window.clone();
            edit_monster_button.connect_clicked(move |_| {
                edit_monster_creation_menu(
                    &app_clone_for_edit,
                    &window_clone_for_edit,
                    monster_for_edit.clone(),
                )
            });

            let monster_name_for_attack = monster.name.clone();
            let app_clone_for_attack = app.clone();
            let window_clone_for_attack = window.clone();
            add_attack_button.connect_clicked(move |_| {
                show_attack_creation_menu(
                    &app_clone_for_attack,
                    &window_clone_for_attack,
                    &monster_name_for_attack,
                );
            });
            
            let monster_name_for_remove = monster.name.clone();
            let app_clone_for_remove = app.clone();
            let window_clone_for_remove = window.clone();
            remove_attack_button.connect_clicked(move |_| {
                show_remove_attack_menu(
                    &app_clone_for_remove,
                    &window_clone_for_remove,
                    &monster_name_for_remove,
                );
            });

            let monster_name_to_delete = monster.name.clone();
            let app_clone_for_refresh = app.clone();
            let window_clone_for_refresh = window.clone();
            delete_button.connect_clicked(move |_| {
                if let Err(e) = monster_manager::delete_monster(&monster_name_to_delete) {
                    eprintln!("Failed to delete monster '{}': {}", monster_name_to_delete, e);
                }
                switch_to_monster_list(&app_clone_for_refresh, &window_clone_for_refresh);
            });

            // Assemble the row and append to the list box.
            row.append(&info_vbox);
            row.append(&button_box);
            list_box.append(&row);
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    window.set_content(Some(&main_vbox));
    window.present();
}

// Helper function to create a labeled Entry widget pair.
fn create_label_entry_pair(label_text: &str, placeholder_text: &str) -> (Label, Entry) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();

    let entry = Entry::builder()
        .placeholder_text(placeholder_text)
        .build();

    (label, entry)
}

fn create_text_label_entry_pair(label_text: &str, text: &str) -> (Label, Entry) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();

    let entry = Entry::builder()
        .text(text)
        .build();

    (label, entry)
}

// Helper function to create a labeled DropDown widget pair.
fn create_label_dropdown_pair(label_text: &str, items: &[&str]) -> (Label, DropDown) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();
    
    let string_list = StringList::new(items);

    let dropdown = DropDown::builder()
        .model(&string_list)
        .build();

    (label, dropdown)
}

// Displays a modal window for adding a new attack to a specific monster.
pub fn show_attack_creation_menu(app: &AdwApplication, parent_window: &AdwWindow, monster_name: &str) {
    let window = AdwWindow::builder()
        .application(app)
        .title(&format!("Add Attack to {}", monster_name))
        .transient_for(parent_window)
        .default_width(400)
        .default_height(350)
        .modal(true)
        .build();

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title_label = Label::builder()
        .label(&format!("Add Attack to {}", monster_name))
        .halign(Align::Center)
        .build();
    title_label.add_css_class("title-1");

    let input_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();

    let (attack_name_label, attack_name_entry) = create_label_entry_pair("Attack Name:", "e.g., Bite");
    let (ability_label, ability_dropdown) = create_label_dropdown_pair("Ability Used:", &["str", "dex", "con", "int", "wis", "cha"]);
    let (dice_used_label, dice_used_dropdown) = create_label_dropdown_pair("Dice Used:", &["d4", "d6", "d8", "d10", "d12"]);
    let (num_dice_label, num_dice_entry) = create_label_entry_pair("Number of Dice:", "e.g., 2");
    let (num_attacks_label, num_attacks_entry) = create_label_entry_pair("Attacks per Turn:", "e.g., 1");
    
    input_grid.attach(&attack_name_label, 0, 0, 1, 1);
    input_grid.attach_next_to(&attack_name_entry, Some(&attack_name_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&ability_label, 0, 1, 1, 1);
    input_grid.attach_next_to(&ability_dropdown, Some(&ability_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&dice_used_label, 0, 2, 1, 1);
    input_grid.attach_next_to(&dice_used_dropdown, Some(&dice_used_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&num_dice_label, 0, 3, 1, 1);
    input_grid.attach_next_to(&num_dice_entry, Some(&num_dice_label), gtk::PositionType::Right, 1, 1);
    input_grid.attach(&num_attacks_label, 0, 4, 1, 1);
    input_grid.attach_next_to(&num_attacks_entry, Some(&num_attacks_label), gtk::PositionType::Right, 1, 1);
    
    let error_label = Label::builder()
        .label("")
        .halign(Align::Center)
        .build();

    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::End)
        .build();
    let save_button = Button::with_label("Save Attack");
    let cancel_button = Button::with_label("Cancel");
    button_box.append(&save_button);
    button_box.append(&cancel_button);

    main_vbox.append(&title_label);
    main_vbox.append(&input_grid);
    main_vbox.append(&error_label);
    main_vbox.append(&button_box);

    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    let monster_name_clone = monster_name.to_string();
    let error_label_clone = error_label.clone();
    let attack_name_entry_clone = attack_name_entry.clone();
    let ability_dropdown_clone = ability_dropdown.clone();
    let dice_used_dropdown_clone = dice_used_dropdown.clone();
    let num_dice_entry_clone = num_dice_entry.clone();
    let num_attacks_entry_clone = num_attacks_entry.clone();

    save_button.connect_clicked(move |_| {
        error_label_clone.set_text("");

        let get_dropdown_text = |dropdown: &DropDown| {
            dropdown.selected_item()
                .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
                .unwrap_or_default()
        };
        
        let attack_name = attack_name_entry_clone.text().to_string();
        let ability_used = get_dropdown_text(&ability_dropdown_clone);
        let dice_used = get_dropdown_text(&dice_used_dropdown_clone);
        let num_dice: i32 = match num_dice_entry_clone.text().parse() {
            Ok(n) => n,
            Err(_) => {
                error_label_clone.set_text("Number of Dice must be a valid number.");
                return;
            }
        };
        let num_attacks: i32 = match num_attacks_entry_clone.text().parse() {
            Ok(n) => n,
            Err(_) => {
                error_label_clone.set_text("Attacks per Turn must be a valid number.");
                return;
            }
        };

        if attack_name.trim().is_empty() || ability_used.is_empty() || dice_used.is_empty() || num_dice <= 0 || num_attacks <= 0 {
            error_label_clone.set_text("Please fill all fields correctly.");
            return;
        }

        let new_attack = monster_manager::Attack {
            attack_name,
            ability_used,
            dice_used,
            num_dice,
            num_attacks,
        };

        if let Err(e) = monster_manager::add_attack_to_monster(&monster_name_clone, new_attack) {
            error_label_clone.set_text(&format!("Failed to save attack: {}", e));
            return;
        }

        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    window.set_content(Some(&main_vbox));
    window.present();
}

// Displays a modal window for removing an attack from a monster.
fn show_remove_attack_menu(app: &AdwApplication, parent_window: &AdwWindow, monster_name: &str) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Remove Attack")
        .transient_for(parent_window)
        .modal(true)
        .default_width(350)
        .default_height(400)
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
        .label("Select Attack to Remove")
        .halign(Align::Center)
        .build();
    title.add_css_class("title-2");
    main_vbox.append(&title);

    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    
    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();
    list_box.add_css_class("boxed-list");

    let monster_data = monster_manager::read_monster(monster_name);
    if let Some(monster) = monster_data {
        if monster.attacks.is_empty() {
            list_box.append(&Label::new(Some("This monster has no attacks to remove.")));
        } else {
            for attack in &monster.attacks {
                let row = create_remove_attack_row(
                    attack,
                    &monster.name,
                    window.clone(),
                    parent_window.clone(),
                    app.clone(),
                );
                list_box.append(&row);
            }
        }
    } else {
        list_box.append(&Label::new(Some("Monster not found.")));
    }
    
    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let close_button = Button::builder()
        .label("Close")
        .halign(Align::End)
        .build();
    
    let window_clone = window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });

    main_vbox.append(&close_button);
    window.set_content(Some(&main_vbox));
    window.present();
}

// Helper function to create a row for removing an attack.
fn create_remove_attack_row(attack: &monster_manager::Attack, monster_name: &str, modal_window: AdwWindow, parent_window: AdwWindow, app: AdwApplication) -> Box {
    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let attack_name = Label::builder()
        .label(&attack.attack_name)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let remove_button = Button::builder()
        .label("Remove")
        .build();
    remove_button.add_css_class("destructive-action");
    
    let attack_name_clone = attack.attack_name.clone();
    let monster_name_clone = monster_name.to_string();

    remove_button.connect_clicked(move |_| {
        if let Err(e) = monster_manager::delete_attack_from_monster(&monster_name_clone, &attack_name_clone) {
            eprintln!("Failed to delete attack: {}", e);
        }
        
        modal_window.close();
        switch_to_monster_list(
            &app,
            &parent_window,
        );
    });
    
    hbox.append(&attack_name);
    hbox.append(&remove_button);
    hbox
}
