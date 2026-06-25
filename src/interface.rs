// interface.rs
//
// This file manages the user interface for the Mass Combat Decider application.

use std::rc::Rc;
use std::cell::{ Cell, RefCell };
use gtk::{ Entry, Label, ListBox, Orientation, TextView, pango, FlowBox, prelude::* };
use gtk::{ Button, Align, Box };
use libadwaita::Application as AdwApplication;
use gtk::ApplicationWindow as AdwWindow;

use crate::monster_manager::Monster;
use crate::ui_factory::UiFactory;

use super::{ monster_manager, simulation };

// =========================================================================
// Monster Creation/Editing Form
// =========================================================================

/// New monster creation menu
pub fn show_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow) {
    show_monster_form(app, parent_window, None);
}

/// Edit monster creation menu
pub fn edit_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow, monster: Monster) {
    show_monster_form(app, parent_window, Some(monster));
}

/// The messy display logic that actually creates the gui for it
fn show_monster_form(app: &AdwApplication, parent_window: &AdwWindow, existing_monster: Option<Monster>) {
    let is_edit = existing_monster.is_some();

    // Unpack data fields based on create/edit mode
    let (name, hp, ac, speed, exp, pb, hitdie_idx, mods, saves, vulns, rests, immuns, abilities) =
        match &existing_monster {
            Some(m) =>
                (
                    m.name.clone(),
                    m.hp.to_string(),
                    m.ac.to_string(),
                    m.speed.to_string(),
                    m.exp.to_string(),
                    m.pb.to_string(),
                    ["d4", "d6", "d8", "d10", "d12", "d20"]
                        .iter()
                        .position(|&x| x == m.hitdie)
                        .unwrap_or(0) as u32,
                    m.mods,
                    m.saves,
                    m.vulnerabilities.clone(),
                    m.restistances.clone(),
                    m.immunities.clone(),
                    m.abilities.clone(),
                ),
            None =>
                (
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    0,
                    [0; 6],
                    [false; 6],
                    vec![],
                    vec![],
                    vec![],
                    "".to_string(),
                ),
        };
    
    let title_text = if is_edit { "Edit Monster" } else { "Create a Monster" };
    let submit_btn_text = if is_edit { "Edit Monster" } else { "Create Monster" };

    let window = AdwWindow::builder()
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(800)
        .modal(true)
        .build();

    let header_bar = libadwaita::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let big_vbox = UiFactory::create_box(Orientation::Vertical, 12, (24, 24, 24, 24));
    let title_label = UiFactory::create_label(title_text, Align::Center, false, &["title-1"]);

    // --- Top Core Grid ---
    let top_grid = UiFactory::create_grid(12, 12, Align::Center);

    let monster_name_label = UiFactory::create_label("Monster Name:", Align::Start, false, &[]);
    let monster_name_entry = UiFactory::create_entry(Some(&name), Some("Enter name..."), 15);

    let hp_label = UiFactory::create_label("HP:", Align::Start, false, &[]);
    let hp_entry = UiFactory::create_entry(Some(&hp), Some("Enter hp..."), 15);

    let ac_label = UiFactory::create_label("AC:", Align::Start, false, &[]);
    let ac_entry = UiFactory::create_entry(Some(&ac), Some("ac..."), 5);

    let speed_label = UiFactory::create_label("Speed:", Align::Start, false, &[]);
    let speed_entry = UiFactory::create_entry(Some(&speed), Some("speed..."), 7);

    let exp_label = UiFactory::create_label("EXP:", Align::Start, false, &[]);
    let exp_entry = UiFactory::create_entry(Some(&exp), Some("Enter xp..."), 15);

    let pb_label = UiFactory::create_label("PB:", Align::Start, false, &[]);
    let pb_entry = UiFactory::create_entry(Some(&pb), Some("Enter pb..."), 15);

    let die_label = UiFactory::create_label("Die:", Align::Start, false, &[]);
    let dice_options = ["d4", "d6", "d8", "d10", "d12", "d20"];
    let die_dropdown = UiFactory::create_dropdown(&dice_options, Some(hitdie_idx), Some(15));

    let ac_speed_block = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
    ac_speed_block.append(&ac_entry);
    ac_speed_block.append(&speed_label);
    ac_speed_block.append(&speed_entry);

    // Row 0 Layout
    top_grid.attach(&monster_name_label, 0, 0, 1, 1);
    top_grid.attach(&monster_name_entry, 1, 0, 1, 1);
    top_grid.attach(&hp_label, 2, 0, 1, 1);
    top_grid.attach(&hp_entry, 3, 0, 1, 1);
    top_grid.attach(&ac_label, 4, 0, 1, 1);
    top_grid.attach(&ac_speed_block, 5, 0, 1, 1);
    // Row 1 Layout
    top_grid.attach(&exp_label, 0, 1, 1, 1);
    top_grid.attach(&exp_entry, 1, 1, 1, 1);
    top_grid.attach(&pb_label, 2, 1, 1, 1);
    top_grid.attach(&pb_entry, 3, 1, 1, 1);
    top_grid.attach(&die_label, 4, 1, 1, 1);
    top_grid.attach(&die_dropdown, 5, 1, 1, 1);

    // --- Lower Layout Columns ---
    let lower_hbox = UiFactory::create_box(Orientation::Horizontal, 12, (24, 0, 24, 24));
    lower_hbox.set_halign(Align::Center);

    let left_vbox = UiFactory::create_box(Orientation::Vertical, 12, (24, 0, 24, 24));
    left_vbox.set_halign(Align::Center);
    
    // --- Modifier and Saving Throw Grid ---
    let mod_grid = UiFactory::create_grid(12, 12, Align::Center);
    let mod_label = UiFactory::create_label("Mods", Align::Center, false, &[]);
    let save_label = UiFactory::create_label("Save Prof", Align::Center, false, &[]);

    let stats_data = [
        ("Str:", mods[0], saves[0]),
        ("Dex:", mods[1], saves[1]),
        ("Con:", mods[2], saves[2]),
        ("Int:", mods[3], saves[3]),
        ("Wis:", mods[4], saves[4]),
        ("Cha:", mods[5], saves[5]),
    ];

    let mut mod_entries = Vec::new();
    let mut save_checks = Vec::new();

    for (i, &(lbl_text, m_val, s_val)) in stats_data.iter().enumerate() {
        let label = UiFactory::create_label(lbl_text, Align::Start, false, &[]);
        let entry = UiFactory::create_entry(Some(&m_val.to_string()), None, 3);
        let check = UiFactory::create_check_button(s_val);

        mod_grid.attach(&label, 0, (i + 1) as i32, 1, 1);
        mod_grid.attach(&entry, 1, (i + 1) as i32, 1, 1);
        mod_grid.attach(&check, 2, (i + 1) as i32, 1, 1);

        mod_entries.push(entry);
        save_checks.push(check);
    }
    mod_grid.attach(&mod_label, 1, 0, 1, 1);
    mod_grid.attach(&save_label, 2, 0, 1, 1);

    let make_monster_button = UiFactory::create_button(submit_btn_text, Align::Center, None);
    let cancel_button = UiFactory::create_button("Cancel", Align::Center, None);
    let save_hbox = UiFactory::create_box(Orientation::Horizontal, 12, (0, 0, 0, 0));
    save_hbox.append(&make_monster_button);
    save_hbox.append(&cancel_button);

    left_vbox.append(&mod_grid);
    left_vbox.append(&save_hbox);

    // --- Right Column (Resistances & Abilities) ---
    let right_vbox = UiFactory::create_box(Orientation::Vertical, 12, (24, 0, 24, 24));
    right_vbox.set_width_request(475);

    let res_label = UiFactory::create_label("Resistances", Align::Start, false, &["title-3"]);

    let has_initial_res = vulns.len() + rests.len() + immuns.len() > 0;
    let no_res_options = Rc::new(Cell::new(!has_initial_res));

    let selected_vulns = Rc::new(RefCell::new(vulns));
    let selected_res = Rc::new(RefCell::new(rests));
    let selected_imun = Rc::new(RefCell::new(immuns));

    let res_options = [
        "Acid",
        "Bludgening",
        "Cold",
        "Fire",
        "Force",
        "Lightning",
        "Necrotic",
        "Piercing",
        "Poison",
        "Psychic",
        "Radiant",
        "Slashing",
        "Thunder",
    ];
    let res_dropdown = UiFactory::create_dropdown(&res_options, None, Some(30));
    res_dropdown.set_margin_end(170);

    let button_hbox = UiFactory::create_box(Orientation::Horizontal, 12, (0, 0, 0, 0));
    let vuln_button = UiFactory::create_button("Vulnerable", Align::Start, None);
    let res_button = UiFactory::create_button("Resistant", Align::Start, None);
    let immun_button = UiFactory::create_button("Immune", Align::Start, None);

    button_hbox.append(&vuln_button);
    button_hbox.append(&res_button);
    button_hbox.append(&immun_button);

    let flow_box = FlowBox::builder()
        .valign(Align::Start)
        .max_children_per_line(2)
        .min_children_per_line(2)
        .selection_mode(gtk::SelectionMode::None)
        .row_spacing(5)
        .column_spacing(1)
        .build();

    let no_res_label = UiFactory::create_label("No Resistances", Align::Start, false, &[]);

    // Load initial chip data if available
    if has_initial_res {
        for v in selected_vulns.borrow().clone() {
            add_resistance_chip(
                &flow_box,
                Rc::clone(&selected_vulns),
                &[Rc::clone(&selected_res), Rc::clone(&selected_imun)],
                v,
                "Vulnerable",
                &no_res_options,
                &no_res_label
            );
        }
        for r in selected_res.borrow().clone() {
            add_resistance_chip(
                &flow_box,
                Rc::clone(&selected_res),
                &[Rc::clone(&selected_vulns), Rc::clone(&selected_imun)],
                r,
                "Resistant",
                &no_res_options,
                &no_res_label
            );
        }
        for i in selected_imun.borrow().clone() {
            add_resistance_chip(
                &flow_box,
                Rc::clone(&selected_imun),
                &[Rc::clone(&selected_vulns), Rc::clone(&selected_res)],
                i,
                "imunerable",
                &no_res_options,
                &no_res_label
            );
        }
    } else {
        flow_box.insert(&no_res_label, -1);
    }

    let abil_label = UiFactory::create_label("Abilities", Align::Start, false, &["title-3"]);
    let abil_entry = TextView::builder()
        .editable(true)
        .focusable(true)
        .accepts_tab(true)
        .hexpand(true)
        .vexpand(false)
        .build();
    abil_entry.add_css_class("view");
    abil_entry.grab_focus();
    if is_edit {
        abil_entry.buffer().set_text(&abilities);
    }

    let scrolled_container = UiFactory::create_scrolled_window(false, false, Some(150));
    scrolled_container.set_width_request(300);
    scrolled_container.set_child(Some(&abil_entry));
    scrolled_container.set_has_frame(true);
    scrolled_container.add_css_class("frame");

    right_vbox.append(&res_label);
    right_vbox.append(&res_dropdown);
    right_vbox.append(&button_hbox);
    right_vbox.append(&flow_box);
    right_vbox.append(&abil_label);
    right_vbox.append(&scrolled_container);

    lower_hbox.append(&left_vbox);
    lower_hbox.append(&right_vbox);

    big_vbox.append(&title_label);
    big_vbox.append(&top_grid);
    big_vbox.append(&lower_hbox);

    // --- Wire Up Chip Insertion Controls ---
    let flow_box_clone = flow_box.clone();
    let res_dropdown_clone = res_dropdown.clone();
    let no_res_label_clone = no_res_label.clone();
    let no_res_options_clone = Rc::clone(&no_res_options);
    let selected_vulns_clone = Rc::clone(&selected_vulns);
    let selected_res_clone = Rc::clone(&selected_res);
    let selected_imun_clone = Rc::clone(&selected_imun);
    vuln_button.connect_clicked(move |_| {
        let term = UiFactory::get_dropdown_text(&res_dropdown_clone);
        add_resistance_chip(
            &flow_box_clone,
            Rc::clone(&selected_vulns_clone),
            &[Rc::clone(&selected_res_clone), Rc::clone(&selected_imun_clone)],
            term,
            "Vulnerable",
            &no_res_options_clone,
            &no_res_label_clone
        );
    });

    let flow_box_clone = flow_box.clone();
    let res_dropdown_clone = res_dropdown.clone();
    let no_res_label_clone = no_res_label.clone();
    let no_res_options_clone = Rc::clone(&no_res_options);
    let selected_vulns_clone = Rc::clone(&selected_vulns);
    let selected_res_clone = Rc::clone(&selected_res);
    let selected_imun_clone = Rc::clone(&selected_imun);
    res_button.connect_clicked(move |_| {
        let term = UiFactory::get_dropdown_text(&res_dropdown_clone);
        add_resistance_chip(
            &flow_box_clone,
            Rc::clone(&selected_res_clone),
            &[Rc::clone(&selected_vulns_clone), Rc::clone(&selected_imun_clone)],
            term,
            "Resistance",
            &no_res_options_clone,
            &no_res_label_clone
        );
    });

    let flow_box_clone = flow_box.clone();
    let res_dropdown_clone = res_dropdown.clone();
    let no_res_label_clone = no_res_label.clone();
    let no_res_options_clone = Rc::clone(&no_res_options);
    let selected_vulns_clone = Rc::clone(&selected_vulns);
    let selected_res_clone = Rc::clone(&selected_res);
    let selected_imun_clone = Rc::clone(&selected_imun);
    immun_button.connect_clicked(move |_| {
        let term = UiFactory::get_dropdown_text(&res_dropdown_clone);
        add_resistance_chip(
            &flow_box_clone,
            Rc::clone(&selected_imun_clone),
            &[Rc::clone(&selected_vulns_clone), Rc::clone(&selected_res_clone)],
            term,
            "Immune",
            &no_res_options_clone,
            &no_res_label_clone
        );
    });

    // --- Save Handler ---
    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    let monster_name_entry_clone = monster_name_entry.clone();
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let speed_entry_clone = speed_entry.clone();
    let exp_entry_clone = exp_entry.clone();
    let pb_entry_clone = pb_entry.clone();
    let die_dropdown_clone = die_dropdown.clone();
    let mod_entries_clone = mod_entries.clone();
    let save_checks_clone = save_checks.clone();
    let selected_vulns_save = Rc::clone(&selected_vulns);
    let selected_res_save = Rc::clone(&selected_res);
    let selected_imun_save = Rc::clone(&selected_imun);
    let abil_entry_clone = abil_entry.clone();
    let existing_monster_for_save = existing_monster.clone();

    make_monster_button.connect_clicked(move |_| {
        let parse_int = |entry: &Entry| -> i32 { entry.text().parse::<i32>().unwrap_or(0) };

        let name = monster_name_entry_clone.text().to_string();
        if name.trim().is_empty() {
            return;
        }

        let hp = parse_int(&hp_entry_clone);
        let ac = parse_int(&ac_entry_clone);
        let speed = parse_int(&speed_entry_clone);
        let exp = parse_int(&exp_entry_clone);
        let pb = parse_int(&pb_entry_clone);

        let mods: [i32; 6] = [
            parse_int(&mod_entries_clone[0]),
            parse_int(&mod_entries_clone[1]),
            parse_int(&mod_entries_clone[2]),
            parse_int(&mod_entries_clone[3]),
            parse_int(&mod_entries_clone[4]),
            parse_int(&mod_entries_clone[5]),
        ];

        let saves: [bool; 6] = [
            save_checks_clone[0].is_active(),
            save_checks_clone[1].is_active(),
            save_checks_clone[2].is_active(),
            save_checks_clone[3].is_active(),
            save_checks_clone[4].is_active(),
            save_checks_clone[5].is_active(),
        ];

        let hitdie = UiFactory::get_dropdown_text(&die_dropdown_clone);
        let abilities = abil_entry_clone
            .buffer()
            .text(
                &abil_entry_clone.buffer().start_iter(),
                &abil_entry_clone.buffer().end_iter(),
                false
            );

        let new_monster = Monster {
            name,
            hp,
            ac,
            exp,
            pb,
            speed,
            hitdie,
            mods,
            saves,
            vulnerabilities: selected_vulns_save.borrow().clone(),
            restistances: selected_res_save.borrow().clone(),
            immunities: selected_imun_save.borrow().clone(),
            abilities: abilities.to_string(),
            attacks: if is_edit {
                existing_monster_for_save
                    .as_ref()
                    .map(|m| m.attacks.clone())
                    .unwrap_or_default()
            } else {
                Vec::new()
            },
        };

        if let Err(e) = monster_manager::save_monster(new_monster) {
            println!("Failed to save monster: {}", e);
            return;
        }

        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    let window_cancel_clone = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_cancel_clone.close();
    });

    window.set_child(Some(&big_vbox));
    window.present();
}

/// Helper function to build a clean resistance tag chip and manage UI changes and backing state vectors.
fn add_resistance_chip(flow_box: &FlowBox, list: Rc<RefCell<Vec<String>>>, other_lists: &[Rc<RefCell<Vec<String>>>], term: String,label_suffix: &str, no_res_options: &Rc<Cell<bool>>, no_res_label: &Label) {
    if term.trim().is_empty() {
        return;
    }

    // Remove fallback placeholder label on first chip load
    if no_res_options.get() {
        flow_box.remove(no_res_label);
        no_res_options.set(false);
    }

    // Stop duplicate insertions into any resistance category
    if list.borrow().contains(&term) || other_lists.iter().any(|lst| lst.borrow().contains(&term)) {
        return;
    }

    list.borrow_mut().push(term.clone());
    list.borrow_mut().sort();

    let surrounding_hbox = UiFactory::create_box(Orientation::Horizontal, 12, (0, 0, 0, 0));
    let chip_label = UiFactory::create_label(
        &format!("{}: ({})", term, label_suffix),
        Align::Start,
        false,
        &[]
    );
    let remove_button = UiFactory::create_button("x", Align::End, None);
    remove_button.set_width_request(5);

    surrounding_hbox.append(&chip_label);
    surrounding_hbox.append(&remove_button);

    let list_clone = Rc::clone(&list);
    let term_clone = term.clone();
    let surrounding_hbox_clone = surrounding_hbox.clone();
    let flow_box_clone = flow_box.clone();

    remove_button.connect_clicked(move |_| {
        let mut target_list = list_clone.borrow_mut();
        if let Some(flowbox_child) = surrounding_hbox_clone.parent() {
            flow_box_clone.remove(&flowbox_child);
            if let Ok(idx) = target_list.binary_search(&term_clone) {
                target_list.remove(idx);
            }
        }
    });

    flow_box.insert(&surrounding_hbox, -1);
}

// =========================================================================
// Secondary Views (Welcome View + Monster list)
// =========================================================================

/// gui shown when first booting up the app
pub fn switch_to_first_time(app: &AdwApplication, window: &AdwWindow) {
    let welcome = UiFactory::create_label(
        "Welcome to the Mass Combat Decider.\nThis app is designed to assist you in making combat easier and faster for the DM. It's meant to provide quick calculations for things like # of attacks, attack, and damage rolls.\n\nSince you don't have any monsters yet, create your first one using the button below!",
        Align::Center,
        false,
        &[]
    );
    welcome.set_margin_top(12);
    welcome.set_margin_start(12);
    welcome.set_margin_end(12);
    welcome.set_wrap(true);

    let create_monster_button = UiFactory::create_button(
        "Create Monster",
        Align::Center,
        Some("suggested-action")
    );
    create_monster_button.set_margin_top(12);

    let app_clone = app.clone();
    let window_clone = window.clone();
    create_monster_button.connect_clicked(move |_| {
        show_monster_creation_menu(&app_clone, &window_clone);
    });

    let vbox = UiFactory::create_box(Orientation::Vertical, 6, (0, 0, 0, 0));
    vbox.set_halign(Align::Center);

    vbox.append(&welcome);
    vbox.append(&create_monster_button);

    window.set_child(Some(&vbox));
    window.present();
}

/// gui shown opppening the app normally
pub fn switch_to_monster_list(app: &AdwApplication, window: &AdwWindow) {
    window.set_title(Some("Mass Combat Decider - Your Monsters"));

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title_label = UiFactory::create_label("Your Monsters", Align::Center, false, &["title-1"]);
    title_label.set_margin_bottom(12);

    let top_button_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 12, 0, 0));
    top_button_box.set_halign(Align::Center);

    let create_monster_button = UiFactory::create_button("Create New Monster", Align::Center, None);
    let start_simulation_button = UiFactory::create_button(
        "Start Simulation",
        Align::Center,
        Some("suggested-action")
    );
    let continue_simulation_button = UiFactory::create_button(
        "Continue Simulation",
        Align::Center,
        None
    );

    top_button_box.append(&create_monster_button);
    top_button_box.append(&start_simulation_button);
    if simulation::check_for_simulation() {
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
        simulation::start_simulation_view(
            &app_clone_continue_sim,
            &window_clone_continue_sim,
            Vec::new()
        );
    });

    let scrolled_window = UiFactory::create_scrolled_window(true, true, None);
    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
    list_box.add_css_class("boxed-list");

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
            let row = UiFactory::create_box(Orientation::Horizontal, 12, (6, 6, 12, 12));

            let info_vbox = UiFactory::create_box(Orientation::Vertical, 3, (0, 0, 0, 0));
            info_vbox.set_halign(Align::Start);
            info_vbox.set_hexpand(true);

            let name_label = UiFactory::create_label(
                &format!("<b>{}</b>", monster.name),
                Align::Start,
                true,
                &[]
            );
            let stats_label = UiFactory::create_label(
                &format!(
                    "HP: {}, AC: {}, EXP: {}, PB: {}, \nSTR: {}, DEX: {}, CON: {}, INT: {}, WIS: {}, CHA: {}",
                    monster.hp,
                    monster.ac,
                    monster.exp,
                    monster.pb,
                    monster.mods[0],
                    monster.mods[1],
                    monster.mods[2],
                    monster.mods[3],
                    monster.mods[4],
                    monster.mods[5]
                ),
                Align::Start,
                false,
                &[]
            );

            let attacks_str = monster.attacks
                .iter()
                .map(|a| a.attack_name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");

            let attacks_label = UiFactory::create_label(
                &format!("Attacks: {}", if attacks_str.is_empty() { "None" } else { &attacks_str }),
                Align::Start,
                false,
                &[]
            );
            attacks_label.set_ellipsize(pango::EllipsizeMode::End);
            attacks_label.set_tooltip_text(Some(&attacks_str));

            info_vbox.append(&name_label);
            info_vbox.append(&stats_label);
            info_vbox.append(&attacks_label);

            let button_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
            button_box.set_halign(Align::End);

            let edit_monster_button = Button::with_label("Edit");
            let add_attack_button = Button::with_label("Add Attack");
            let remove_attack_button = Button::with_label("Remove Attack");
            let delete_button = Button::with_label("Delete");
            delete_button.add_css_class("destructive-action");

            button_box.append(&edit_monster_button);
            button_box.append(&add_attack_button);
            button_box.append(&remove_attack_button);
            button_box.append(&delete_button);

            let monster_for_edit = monster.clone();
            let app_clone_for_edit = app.clone();
            let window_clone_for_edit = window.clone();
            edit_monster_button.connect_clicked(move |_| {
                edit_monster_creation_menu(
                    &app_clone_for_edit,
                    &window_clone_for_edit,
                    monster_for_edit.clone()
                )
            });

            let monster_name_for_attack = monster.name.clone();
            let app_clone_for_attack = app.clone();
            let window_clone_for_attack = window.clone();
            add_attack_button.connect_clicked(move |_| {
                show_attack_creation_menu(
                    &app_clone_for_attack,
                    &window_clone_for_attack,
                    &monster_name_for_attack
                );
            });

            let monster_name_for_remove = monster.name.clone();
            let app_clone_for_remove = app.clone();
            let window_clone_for_remove = window.clone();
            remove_attack_button.connect_clicked(move |_| {
                show_remove_attack_menu(
                    &app_clone_for_remove,
                    &window_clone_for_remove,
                    &monster_name_for_remove
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

            row.append(&info_vbox);
            row.append(&button_box);
            list_box.append(&row);
        }
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    window.set_child(Some(&main_vbox));
    window.present();
}

// =========================================================================
// Attack Creation / Removal Forms
// =========================================================================

/// Displays the form used to create attacks
pub fn show_attack_creation_menu(app: &AdwApplication, parent_window: &AdwWindow, monster_name: &str) {
    let window = AdwWindow::builder()
        .application(app)
        .title(&format!("Add Attack to {}", monster_name))
        .transient_for(parent_window)
        .default_width(400)
        .default_height(350)
        .modal(true)
        .build();

    let header_bar = libadwaita::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title_label = UiFactory::create_label(
        &format!("Add Attack to {}", monster_name),
        Align::Center,
        false,
        &["title-1"]
    );

    let input_grid = UiFactory::create_grid(12, 12, Align::Center);

    let (attack_name_label, attack_name_entry) = UiFactory::create_label_entry_pair(
        "Attack Name:",
        "e.g., Bite"
    );
    let (ability_label, ability_dropdown) = UiFactory::create_label_dropdown_pair(
        "Ability Used:",
        &["str", "dex", "con", "int", "wis", "cha"]
    );
    let (dice_used_label, dice_used_dropdown) = UiFactory::create_label_dropdown_pair(
        "Dice Used:",
        &["d4", "d6", "d8", "d10", "d12"]
    );
    let (num_dice_label, num_dice_entry) = UiFactory::create_label_entry_pair(
        "Number of Dice:",
        "e.g., 2"
    );
    let (num_attacks_label, num_attacks_entry) = UiFactory::create_label_entry_pair(
        "Attacks per Turn:",
        "e.g., 1"
    );
    let (saving_throw_label, saving_throw_checkbox) =
        UiFactory::create_label_checkbox_pair("Is this a saving throw?");

    input_grid.attach(&attack_name_label, 0, 0, 1, 1);
    input_grid.attach_next_to(
        &attack_name_entry,
        Some(&attack_name_label),
        gtk::PositionType::Right,
        1,
        1
    );
    input_grid.attach(&ability_label, 0, 1, 1, 1);
    input_grid.attach_next_to(
        &ability_dropdown,
        Some(&ability_label),
        gtk::PositionType::Right,
        1,
        1
    );
    input_grid.attach(&dice_used_label, 0, 2, 1, 1);
    input_grid.attach_next_to(
        &dice_used_dropdown,
        Some(&dice_used_label),
        gtk::PositionType::Right,
        1,
        1
    );
    input_grid.attach(&num_dice_label, 0, 3, 1, 1);
    input_grid.attach_next_to(
        &num_dice_entry,
        Some(&num_dice_label),
        gtk::PositionType::Right,
        1,
        1
    );
    input_grid.attach(&num_attacks_label, 0, 4, 1, 1);
    input_grid.attach_next_to(
        &num_attacks_entry,
        Some(&num_attacks_label),
        gtk::PositionType::Right,
        1,
        1
    );
    input_grid.attach(&saving_throw_label, 0, 5, 1, 1);
    input_grid.attach_next_to(
        &saving_throw_checkbox,
        Some(&saving_throw_label),
        gtk::PositionType::Right,
        1,
        1
    );

    let error_label = UiFactory::create_label("", Align::Center, false, &[]);

    let button_box = UiFactory::create_box(Orientation::Horizontal, 6, (0, 0, 0, 0));
    button_box.set_halign(Align::End);
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
    let saving_throw_checkbox_clone = saving_throw_checkbox.clone();

    save_button.connect_clicked(move |_| {
        error_label_clone.set_text("");

        let attack_name = attack_name_entry_clone.text().to_string();
        let ability_used = UiFactory::get_dropdown_text(&ability_dropdown_clone);
        let dice_used = UiFactory::get_dropdown_text(&dice_used_dropdown_clone);
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

        if
            attack_name.trim().is_empty() ||
            ability_used.is_empty() ||
            dice_used.is_empty() ||
            num_dice <= 0 ||
            num_attacks <= 0
        {
            error_label_clone.set_text("Please fill all fields correctly.");
            return;
        }

        let saving_throw = saving_throw_checkbox_clone.is_active();

        let new_attack = monster_manager::Attack {
            attack_name,
            ability_used,
            dice_used,
            num_dice,
            num_attacks,
            saving_throw,
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

    window.set_child(Some(&main_vbox));
    window.present();
}

/// Displays the form used to remove attacks
fn show_remove_attack_menu(app: &AdwApplication, parent_window: &AdwWindow, monster_name: &str) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Remove Attack")
        .transient_for(parent_window)
        .modal(true)
        .default_width(350)
        .default_height(400)
        .build();

    let header_bar = libadwaita::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let main_vbox = UiFactory::create_box(Orientation::Vertical, 12, (12, 12, 12, 12));
    let title = UiFactory::create_label(
        "Select Attack to Remove",
        Align::Center,
        false,
        &["title-2"]
    );
    main_vbox.append(&title);

    let scrolled_window = UiFactory::create_scrolled_window(true, true, None);
    let list_box = ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
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
                    app.clone()
                );
                list_box.append(&row);
            }
        }
    } else {
        list_box.append(&Label::new(Some("Monster not found.")));
    }

    scrolled_window.set_child(Some(&list_box));
    main_vbox.append(&scrolled_window);

    let close_button = UiFactory::create_button("Close", Align::End, None);
    let window_clone = window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });

    main_vbox.append(&close_button);
    window.set_child(Some(&main_vbox));
    window.present();
}

/// Helper function for the attack removal display
fn create_remove_attack_row(attack: &monster_manager::Attack, monster_name: &str, modal_window: AdwWindow, parent_window: AdwWindow, app: AdwApplication) -> Box {
    let hbox = UiFactory::create_box(Orientation::Horizontal, 12, (6, 6, 12, 12));
    let attack_name = UiFactory::create_label(&attack.attack_name, Align::Start, false, &[]);
    attack_name.set_hexpand(true);

    let remove_button = UiFactory::create_button(
        "Remove",
        Align::Center,
        Some("destructive-action")
    );

    let attack_name_clone = attack.attack_name.clone();
    let monster_name_clone = monster_name.to_string();

    remove_button.connect_clicked(move |_| {
        if
            let Err(e) = monster_manager::delete_attack_from_monster(
                &monster_name_clone,
                &attack_name_clone
            )
        {
            eprintln!("Failed to delete attack: {}", e);
        }

        modal_window.close();
        switch_to_monster_list(&app, &parent_window);
    });

    hbox.append(&attack_name);
    hbox.append(&remove_button);
    hbox
}
