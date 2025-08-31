use gtk::{prelude::*, DropDown, Entry, Label, ListBox, Orientation, ScrolledWindow, StringList, StringObject, pango};
use gtk::{Button, Align, Box};
use libadwaita::{prelude::*,};
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;

use super::{monster_manager, simulation};

pub fn show_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow) {
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(500)
        .default_height(500)
        .modal(true)
        .build();


    let title = Label::builder()
        .label("Create a Monster")
        .margin_start(12)
        .margin_top(12)
        .width_request(100)
        .height_request(25)
        .valign(Align::Start)
        .halign(Align::Center)
        .build();


    let left_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .spacing(6)
        .build();

    // put a bunch of lable/entry pairs
    let (monster_name_hbox, monster_name_entry) = create_label_entry_pair("Monster Name:", "Enter monster name...");
    let (hp_hbox, hp_entry) = create_label_entry_pair("HP:", "Enter hit points...");
    let (ac_hbox, ac_entry) = create_label_entry_pair("AC:", "Enter armor class...");
    let (prof_hbox, prof_dropdown) = create_label_dropdown_pair("Proficincy Bonus:", &["1","2","3","4","5","6","7","8"]);
    let (str_hbox, str_dropdown) = create_label_dropdown_pair("Strength Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);

    // Clone the Entry widgets to move them into the closure
    let monster_name_entry_clone = monster_name_entry.clone();
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let prof_dropdown_clone = prof_dropdown.clone();
    let str_dropdown_clone = str_dropdown.clone();

    //apend to vbox
    left_vbox.append(&monster_name_hbox);
    left_vbox.append(&hp_hbox);
    left_vbox.append(&ac_hbox);
    left_vbox.append(&prof_hbox);
    left_vbox.append(&str_hbox);


    let right_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .spacing(6)
        .build();

    // put a bunch of lable/entry pairs
    let (dex_hbox, dex_dropdown) = create_label_dropdown_pair("Dexterity Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);
    let (con_hbox, con_dropdown) = create_label_dropdown_pair("Constitution Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);
    let (int_hbox, int_dropdown) = create_label_dropdown_pair("Intelligence Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);
    let (wis_hbox, wis_dropdown) = create_label_dropdown_pair("Wisdom Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);
    let (cha_hbox, cha_dropdown) = create_label_dropdown_pair("Charisma Mod:", &["-2","-1","0","1","2","3","4","5","6","7","8"]);

    // Clone the Entry widgets to move them into the closure
    let dex_dropdown_clone = dex_dropdown.clone();
    let con_dropdown_clone = con_dropdown.clone();
    let int_dropdown_clone = int_dropdown.clone();
    let wis_dropdown_clone = wis_dropdown.clone();
    let cha_dropdown_clone = cha_dropdown.clone();

    right_vbox.append(&dex_hbox);
    right_vbox.append(&con_hbox);
    right_vbox.append(&int_hbox);
    right_vbox.append(&wis_hbox);
    right_vbox.append(&cha_hbox);

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Center)
        .build();

    hbox.append(&left_vbox);
    hbox.append(&right_vbox);
    
    // finished with the info we need for the monster
    // make save/cancel buttons
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::Center)
        .build();

    let save_button = Button::builder()
        .label("Save Monster")
        .width_request(100)
        .height_request(25)
        .halign(Align::Start)
        .margin_start(12)
        .margin_top(12)
        .build();

    let app_clone_for_refresh = app.clone(); // Clone app for the refresh call
    let parent_window_clone_for_refresh = parent_window.clone(); // Clone parent_window for refresh call
    let window_clone = window.clone();
    save_button.connect_clicked(move |_| {
        println!("Saving Monster");

        // --- Extracting text from Entry widgets ---
        let name = monster_name_entry_clone.text().to_string();
        let hp: i32 = hp_entry_clone.text().parse().unwrap_or_else(|e| {
            eprintln!("Error parsing HP: {}", e);
            0
        });
        let ac: i32 = ac_entry_clone.text().parse().unwrap_or_else(|e| {
            eprintln!("Error parsing AC: {}", e);
            0
        });

        // --- Extracting and parsing from DropDown widgets ---
        let pb: i32 = prof_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Proficiency Bonus from dropdown.");
                0
            });

        let str_mod: i32 = str_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Strength Mod from dropdown.");
                0
            });
        
        let dex_mod: i32 = dex_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Dexterity Mod from dropdown.");
                0
            });

        let con_mod: i32 = con_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Constitution Mod from dropdown.");
                0
            });

        let int_mod: i32 = int_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Intelligence Mod from dropdown.");
                0
            });

        let wis_mod: i32 = wis_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Wisdom Mod from dropdown.");
                0
            });

        let cha_mod: i32 = cha_dropdown_clone.selected_item()
            .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                eprintln!("Error parsing Charisma Mod from dropdown.");
                0
            });


        let new_monster = monster_manager::Monster {
            name,
            hp,
            ac,
            pb,
            str_mod,
            dex_mod,
            con_mod,
            int_mod,
            wis_mod,
            cha_mod,
            attacks: Vec::new(),
        };

        println!("\nAttempting to save Monster: {:?}", new_monster);
        monster_manager::save_monster(new_monster);

        window_clone.close();
        switch_to_monster_list(&app_clone_for_refresh, &parent_window_clone_for_refresh);
    });

    let app_clone_for_refresh_cancel = app.clone();
    let parent_window_clone_for_refresh_cancel = parent_window.clone();
    let cancel_button = Button::builder()
        .label("Cancel")
        .width_request(100)
        .height_request(25)
        .halign(Align::End)
        .margin_start(12)
        .margin_top(12)
        .build();

    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        println!("Monster creation cancelled.");
        window_clone_cancel.close();
        switch_to_monster_list(&app_clone_for_refresh_cancel, &parent_window_clone_for_refresh_cancel);
        
    });

    button_box.append(&save_button);
    button_box.append(&cancel_button);

    let window_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();
    window_box.append(&title);
    window_box.append(&hbox);
    window_box.append(&button_box);

    window.set_content(Some(&window_box));

    // Present the window to the user
    window.present();
}

pub fn switch_to_first_time(app: &AdwApplication, window: &AdwWindow) {
    let welcome = Label::builder()
        .label(
            "Welcome to the Mass Combat Decider.\nThis app is designed entirly to assist you in making combat easier and faster for the dm as controling 4 or more enemies is concidered a mess. This application is only ment to spit out numbers for things like # of attacks their attack and damage rolls taking into account stats for you. \n\n You are seeing this because you dont have any monsters in the system right now. make one in the builder below")
        .margin_top(12)
        .margin_start(12)
        .wrap(true)
        .max_width_chars(100)
        .valign(Align::Start)
        .halign(Align::Start)
        .build();

    let create_monster_button = Button::builder()
        .label("Create Monster")
        .margin_start(12)
        .margin_top(12)
        .halign(Align::Start)
        .width_request(100)
        .height_request(25)
        .build();
    
    let app_clone_for_monster_creation = app.clone(); 
    let window_clone_for_monster_creation = window.clone(); // Clone the main window
    create_monster_button.connect_clicked(move |_| {
        show_monster_creation_menu(&app_clone_for_monster_creation, &window_clone_for_monster_creation);
    });

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

    // 6. Add the text_label and the button to the vertical box.
    vbox.append(&welcome);
    vbox.append(&create_monster_button);

    window.set_content(Some(&vbox));

    // Present the window to the user
    window.present();
}

pub fn switch_to_monster_list(app: &AdwApplication, window: &AdwWindow) {
    // Set the window title for the monster list view
    window.set_title(Some("Mass Combat Decider - Your Monsters"));

    let main_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();

    let title_label = Label::builder()
        .label("Your Monsters")
        .halign(Align::Center)
        .margin_bottom(12)
        .build();

    main_vbox.append(&title_label);

    // Create a box for the top buttons
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
    start_simulation_button.add_css_class("suggested-action"); // Make it stand out

    top_button_box.append(&create_monster_button);
    top_button_box.append(&start_simulation_button);
    main_vbox.append(&top_button_box);

    let app_clone = app.clone();
    let window_clone = window.clone();
    create_monster_button.connect_clicked(move |_| {
        show_monster_creation_menu(&app_clone, &window_clone);
    });

    // Connect the new simulation button
    let app_clone_sim = app.clone();
    let window_clone_sim = window.clone();
    start_simulation_button.connect_clicked(move |_| {
        // This function will be in simulation.rs
        simulation::show_simulation_setup_menu(&app_clone_sim, &window_clone_sim);
    });
    
    // We append the create button again here, because the `top_button_box` seems to be ignored.
    // This is likely a bug in the provided code. I'm adding it back to keep consistency.
    main_vbox.append(&create_monster_button);


    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true) // Allow the scrolled window to expand vertically
        .hexpand(true) // Allow the scrolled window to expand horizontally
        .build();

    let list_box = ListBox::builder()
        .selection_mode(gtk::SelectionMode::None) // No selection needed for display
        .build();
    list_box.add_css_class("boxed-list"); // Adds a nice border around the list


    // Read all monsters from the manager
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
            // We'll use a VBox for each monster row to stack info vertically
            let row_vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(6)
                .margin_start(6)
                .margin_end(6)
                .margin_top(6)
                .margin_bottom(6)
                .build();

            // HBox for Name and primary stats
            let top_hbox = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .build();

            let name_label = Label::builder()
                .label(&format!("<b>{}</b>", monster.name))
                .use_markup(true)
                .halign(Align::Start)
                .hexpand(true)
                .build();

            let stats_label = Label::builder()
                .label(&format!("HP: {}, AC: {}, PB: {}", monster.hp, monster.ac, monster.pb))
                .halign(Align::End)
                .build();

            top_hbox.append(&name_label);
            top_hbox.append(&stats_label);

            // A VBox for secondary info like mods and attacks
            let info_vbox = Box::builder()
                .orientation(Orientation::Vertical)
                .halign(Align::Start)
                .spacing(3)
                .build();
            
            let mods_label = Label::builder()
                .label(&format!("STR:{}, DEX:{}, CON:{}, INT:{}, WIS:{}, CHA:{}",
                                 monster.str_mod, monster.dex_mod, monster.con_mod,
                                 monster.int_mod, monster.wis_mod, monster.cha_mod))
                .halign(Align::Start)
                .build();

            // Create a formatted string of attack names
            let attacks_str = monster.attacks.iter()
                .map(|a| a.attack_name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");

            let attacks_label = Label::builder()
                .label(&format!("Attacks: {}", if attacks_str.is_empty() { "None" } else { &attacks_str }))
                .halign(Align::Start)
                .ellipsize(pango::EllipsizeMode::End) // Truncate if too long
                .tooltip_text(&attacks_str) // Show full list on hover
                .build();

            info_vbox.append(&mods_label);
            info_vbox.append(&attacks_label);


            // HBox for action buttons
            let button_box = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(6)
                .halign(Align::End)
                .build();

            let remove_attack_button = Button::with_label("Remove Attack");
            remove_attack_button.add_css_class("destructive-action");
            
            let add_attack_button = Button::with_label("Add Attack");

            let delete_button = Button::with_label("Delete");
            delete_button.add_css_class("destructive-action"); // Style delete button

            button_box.append(&remove_attack_button);
            button_box.append(&add_attack_button);
            button_box.append(&delete_button);

            // Connect Add Attack button
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

            // Connect Remove Attack button
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

            // Connect Delete button
            let monster_name_to_delete = monster.name.clone();
            let app_clone_for_refresh = app.clone();
            let window_clone_for_refresh = window.clone();
            delete_button.connect_clicked(move |_| {
                println!("Attempting to delete monster: {}", monster_name_to_delete);
                if let Err(e) = monster_manager::delete_monster(&monster_name_to_delete) {
                    eprintln!("Failed to delete monster '{}': {}", monster_name_to_delete, e);
                }
                switch_to_monster_list(&app_clone_for_refresh, &window_clone_for_refresh);
            });
            
            // Assemble the row
            row_vbox.append(&top_hbox);
            row_vbox.append(&info_vbox);
            row_vbox.append(&button_box);

            list_box.append(&row_vbox);
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    window.set_content(Some(&main_vbox));
    window.present();
}

fn create_label_entry_pair(label_text: &str, placeholder_text: &str) -> (Box, Entry) {
    let label = Label::builder()
        .label(label_text)
        .margin_start(12)
        .margin_top(12)
        .width_request(125)
        .height_request(25)
        .valign(Align::Center)
        .halign(Align::Start)
        .build();

    let entry = Entry::builder()
        .margin_top(12)
        .margin_end(12)
        .width_request(150)
        .height_request(25)
        .placeholder_text(placeholder_text)
        .build();

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    hbox.append(&label);
    hbox.append(&entry);

    (hbox, entry) // Return both the hbox and the entry
}

fn create_label_dropdown_pair(label_text: &str, items: &[&str]) -> (Box, DropDown) {
    let label = Label::builder()
        .label(label_text)
        .margin_start(12)
        .margin_top(12)
        .width_request(125)
        .height_request(25)
        .valign(Align::Center)
        .halign(Align::Start)
        .build();

    // Create a StringList from the provided items
    let string_list = StringList::new(items);

    let dropdown = DropDown::builder()
        .model(&string_list) // Set the model for the dropdown
        .margin_top(12)
        .margin_end(12)
        .width_request(100) // Give it some width
        .height_request(25)
        .build();

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .build();
    hbox.append(&label);
    hbox.append(&dropdown);

    (hbox, dropdown) // Return both the hbox and the dropdown
}

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

    // Create the input fields using your helper functions
    let (attack_name_hbox, attack_name_entry) = create_label_entry_pair("Attack Name:", "e.g., Bite");
    let (ability_hbox, ability_dropdown) = create_label_dropdown_pair("Ability Used:", &["str", "dex", "con", "int", "wis", "cha"]);
    let (dice_used_hbox, dice_used_dropdown) = create_label_dropdown_pair("Dice Used:", &["d4", "d6", "d8", "d10", "d12"]);
    let (num_dice_hbox, num_dice_entry) = create_label_entry_pair("Number of Dice:", "e.g., 2");
    let (num_attacks_hbox, num_attacks_entry) = create_label_entry_pair("Attacks per Turn:", "e.g., 1");

    main_vbox.append(&attack_name_hbox);
    main_vbox.append(&ability_hbox);
    main_vbox.append(&dice_used_hbox);
    main_vbox.append(&num_dice_hbox);
    main_vbox.append(&num_attacks_hbox);

    // Save and Cancel buttons
    let button_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .halign(Align::End)
        .build();
    let save_button = Button::with_label("Save Attack");
    let cancel_button = Button::with_label("Cancel");
    button_box.append(&save_button);
    button_box.append(&cancel_button);
    main_vbox.append(&button_box);

    // --- Save button logic ---
    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    let monster_name_clone = monster_name.to_string();
    save_button.connect_clicked(move |_| {
        // Helper closure to get string from a dropdown
        let get_dropdown_text = |dropdown: &DropDown| {
            dropdown.selected_item()
                .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
                .unwrap_or_default()
        };

        let attack_name = attack_name_entry.text().to_string();
        let ability_used = get_dropdown_text(&ability_dropdown);
        let dice_used = get_dropdown_text(&dice_used_dropdown);
        let num_dice: i32 = num_dice_entry.text().parse().unwrap_or(0);
        let num_attacks: i32 = num_attacks_entry.text().parse().unwrap_or(0);

        // Basic validation
        if attack_name.is_empty() || ability_used.is_empty() || dice_used.is_empty() || num_dice <= 0 || num_attacks <= 0 {
            eprintln!("Invalid attack data. Please fill all fields correctly.");
            return; // Don't proceed
        }

        let new_attack = monster_manager::Attack {
            attack_name,
            ability_used,
            dice_used,
            num_dice,
            num_attacks,
        };

        // Add attack to the monster's JSON file
        if let Err(e) = monster_manager::add_attack_to_monster(&monster_name_clone, new_attack) {
            eprintln!("Failed to save attack: {}", e);
        }

        // Close this modal and refresh the main list
        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    // --- Cancel button logic ---
    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    window.set_content(Some(&main_vbox));
    window.present();
}

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

fn create_remove_attack_row( attack: &monster_manager::Attack, monster_name: &str, modal_window: AdwWindow, parent_window: AdwWindow, app: AdwApplication,) -> Box {
    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .margin_top(6).margin_bottom(6).margin_start(6).margin_end(6)
        .build();
    
    let attack_name = Label::builder()
        .label(&attack.attack_name)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let remove_button = Button::builder()
        .label("X")
        .build();
    remove_button.add_css_class("destructive-action");
    
    let attack_name_clone = attack.attack_name.clone();
    let monster_name_clone = monster_name.to_string();

    remove_button.connect_clicked(move |_| {
        if let Err(e) = monster_manager::delete_attack_from_monster(&monster_name_clone, &attack_name_clone) {
            eprintln!("Failed to delete attack: {}", e);
        }
        
        // Refresh the parent window's details page
        switch_to_monster_list(
            &app.clone(),
            &parent_window.clone(),
        );
        
        // Close the modal window after a successful deletion
        modal_window.close();
    });
    
    hbox.append(&attack_name);
    hbox.append(&remove_button);
    hbox
}