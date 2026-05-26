// uimanager.rs
//
// This file manages the user interface for the Mass Combat Decider application.
// It uses GTK4 and Libadwaita to create windows, forms, and lists for managing monsters.

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use gtk::{
    CheckButton, DropDown, Entry, Grid, Label, ListBox, Orientation, ScrolledWindow, StringList, StringObject, TextView, pango, FlowBox ,prelude::*
};
use gtk::{Button, Align, Box};
use libadwaita::{prelude::*,};
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;

use crate::monster_manager::Monster;

use super::{monster_manager, simulation};

// Displays a modal window for creating a new monster.
pub fn show_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(800)
        //.default_height(350)
        .modal(true)
        .build();

    // Main vbox
    let big_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    let title_label = Label::builder()
        .label("Create a Monster")
        .halign(Align::Center)
        .build();
    title_label.add_css_class("title-1");

    

    // top grid section; name, hp, ac, exp, pb, hit die
    let top_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();
    // get stuff for grids
    let (monster_name_label, monster_name_entry) = create_label_entry_pair("Monster Name:", "Enter name...");
    let (hp_label, hp_entry) = create_label_entry_pair("HP:", "Enter hp...");
    let (ac_label, ac_entry) = create_label_entry_pair("AC:", "Enter ac...");
    let (exp_label, exp_entry) = create_label_entry_pair("EXP:", "Enter xp...");
    let (pb_label, pb_entry) = create_label_entry_pair("PB:", "Enter pb...");
    let (die_label, die_dropdown) = create_label_dropdown_pair("Die:", &["d4","d6","d8","d10","d12","d20"]);


    // make the grid. 3 col 2 rows
    // top row
    top_grid.attach(&monster_name_label, 0, 0, 1, 1);
    top_grid.attach(&monster_name_entry, 1, 0, 1, 1);
    top_grid.attach(&hp_label, 2, 0, 1, 1);
    top_grid.attach(&hp_entry, 3, 0, 1, 1);
    top_grid.attach(&ac_label, 4, 0, 1, 1);
    top_grid.attach(&ac_entry, 5, 0, 1, 1);
    // bottom row
    top_grid.attach(&exp_label, 0, 1, 1, 1);
    top_grid.attach(&exp_entry, 1, 1, 1, 1);
    top_grid.attach(&pb_label, 2, 1, 1, 1);
    top_grid.attach(&pb_entry, 3, 1, 1, 1);
    top_grid.attach(&die_label, 4, 1, 1, 1);
    top_grid.attach(&die_dropdown, 5, 1, 1, 1);

    
    // split the lower section into 2 with an hbox
    let lower_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();
    

    // now build mod box
    let left_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();

    let mod_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();
    // labels
    let mod_label = Label::builder().label("Mods").build();
    let save_label = Label::builder().label("Save Prof").build();
    // modifiers
    let (str_label, str_entry, str_check) = create_label_entry_checkbox_triplet("Str:", "0");
    let (dex_label, dex_entry, dex_check) = create_label_entry_checkbox_triplet("Dex:", "0");
    let (con_label, con_entry, con_check) = create_label_entry_checkbox_triplet("Con:", "0");
    let (int_label, int_entry, int_check) = create_label_entry_checkbox_triplet("Int:", "0");
    let (wis_label, wis_entry, wis_check) = create_label_entry_checkbox_triplet("Wis:", "0");
    let (cha_label, cha_entry, cha_check) = create_label_entry_checkbox_triplet("Cha:", "0");

    // append to grid col 3 row 7

    // col 1
    mod_grid.attach(&str_label, 0, 1, 1, 1);
    mod_grid.attach(&dex_label, 0, 2, 1, 1);
    mod_grid.attach(&con_label, 0, 3, 1, 1);
    mod_grid.attach(&int_label, 0, 4, 1, 1);
    mod_grid.attach(&wis_label, 0, 5, 1, 1);
    mod_grid.attach(&cha_label, 0, 6, 1, 1);
    // col 2
    mod_grid.attach(&mod_label, 1, 0, 1, 1);
    mod_grid.attach(&str_entry, 1, 1, 1, 1);
    mod_grid.attach(&dex_entry, 1, 2, 1, 1);
    mod_grid.attach(&con_entry, 1, 3, 1, 1);
    mod_grid.attach(&int_entry, 1, 4, 1, 1);
    mod_grid.attach(&wis_entry, 1, 5, 1, 1);
    mod_grid.attach(&cha_entry, 1, 6, 1, 1);
    // col 3
    mod_grid.attach(&save_label, 2, 0, 1, 1);
    mod_grid.attach(&str_check, 2, 1, 1, 1);
    mod_grid.attach(&dex_check, 2, 2, 1, 1);
    mod_grid.attach(&con_check, 2, 3, 1, 1);
    mod_grid.attach(&int_check, 2, 4, 1, 1);
    mod_grid.attach(&wis_check, 2, 5, 1, 1);
    mod_grid.attach(&cha_check, 2, 6, 1, 1);

    let make_monster_button = Button::builder()
        .label("Create Monster")
        .build();
    let cancel_button = Button::builder()
        .label("Cancel")
        .halign(Align::Center)
        .build();
    let save_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    save_hbox.append(&make_monster_button);
    save_hbox.append(&cancel_button);
    
    left_vbox.append(&mod_grid);
    left_vbox.append(&save_hbox);
    

    // right side v box
    let right_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .width_request(475)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();
    
    let res_label = Label::builder()
        .label("Resistances")
        .halign(Align::Start)
        .build();
    res_label.add_css_class("title-3");

    let no_res_options = Rc::new(Cell::new(true));
    let selected_vulns: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let selected_res: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let selected_imun: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let res_options = ["Acid","Bludgening","Cold","Fire","Force","Lightning","Necrotic", "Piercing", "Poison", "Psychic", "Radiant", "Slashing", "Thunder"];
    let res_dropdown = DropDown::builder()
        .model(&StringList::new(&res_options))
        .width_request(30)
        .margin_end(170)
        .build();

    let button_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    let vuln_button = Button::builder()
        .halign(Align::Start)
        .label("Vulnerable")
        .build();
    let res_button = Button::builder()
        .halign(Align::Start)
        .label("Resistant")
        .build();
    let immun_button = Button::builder()
        .halign(Align::Start)
        .label("Immune")
        .build();

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

    let no_res_label = Label::builder()
        .label("No Resistances")
        .halign(Align::Start)
        .build();

    flow_box.insert(&no_res_label, -1);

    let abil_label = Label::builder()
        .label("Abilities")
        .halign(Align::Start)
        .build();
    abil_label.add_css_class("title-3");

    let abil_entry = TextView::builder()
        .editable(true)
        .focusable(true)
        .accepts_tab(true)
        .hexpand(true)
        .vexpand(false)
        .build();
    abil_entry.add_css_class("view");
    abil_entry.grab_focus();

    let scrolled_container = ScrolledWindow::builder()
        .height_request(150)
        .width_request(300) // Highly recommended so it doesn't squish horizontally
        .child(&abil_entry)
        .has_frame(true) // Enforces the border frame
        .build();

    scrolled_container.add_css_class("frame");

    right_vbox.append(&res_label);
    right_vbox.append(&res_dropdown);
    right_vbox.append(&button_hbox);
    right_vbox.append(&flow_box);
    right_vbox.append(&abil_label);
    right_vbox.append(&scrolled_container);

    // build lower hbox
    lower_hbox.append(&left_vbox);
    lower_hbox.append(&right_vbox);

    // build big vbox
    big_vbox.append(&title_label);
    big_vbox.append(&top_grid);
    big_vbox.append(&lower_hbox);


    // add funcrionality the buttons

    let vuln_dropdown_clone = res_dropdown.clone();
    let vuln_flow_box_clone = flow_box.clone();
    let vuln_no_res_label_clone = no_res_label.clone();
    let vuln_vuln_list_clone= selected_vulns.clone();
    let vuln_res_list_clone= selected_res.clone();
    let vuln_imun_list_clone= selected_imun.clone();
    let vuln_no_res_options_clone = no_res_options.clone();
    vuln_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if vuln_no_res_options_clone.get() {
            vuln_flow_box_clone.remove(&vuln_no_res_label_clone);
            vuln_no_res_options_clone.set(false)
        }

        // 2 check to see if vuln already there
        let vuln_type = get_dropdown_text(&vuln_dropdown_clone);
        let mut selected = vuln_vuln_list_clone.borrow_mut();
        if selected.contains(&vuln_type.clone()) || vuln_res_list_clone.borrow().contains(&vuln_type.clone()) || vuln_imun_list_clone.borrow().contains(&vuln_type.clone()) {return}; // do nothing
        // 3. add vuln to monster
        selected.push(vuln_type.clone());
        selected.sort();

        // 4. add new label button combo
        let vuln_label = Label::builder()
            .label(String::from(vuln_type.clone() + ": (Vulnerable)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&vuln_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = vuln_flow_box_clone.clone();
        let vuln_list_double_clone = vuln_vuln_list_clone.clone();
        let vuln_clone = vuln_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut vuln_list = vuln_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = vuln_list.binary_search(&vuln_clone);
                if indx.is_ok(){
                    vuln_list.remove(indx.unwrap());
                }
            }
        });


        vuln_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    let res_dropdown_clone = res_dropdown.clone();
    let res_flow_box_clone = flow_box.clone();
    let res_no_res_label_clone = no_res_label.clone();
    let res_vuln_list_clone= selected_vulns.clone();
    let res_res_list_clone= selected_res.clone();
    let res_imun_list_clone= selected_imun.clone();
    let res_no_res_options_clone = no_res_options.clone();
    res_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if res_no_res_options_clone.get() {
            res_flow_box_clone.remove(&res_no_res_label_clone);
            res_no_res_options_clone.set(false)
        }

        // 2 check to see if res already there
        let res_type = get_dropdown_text(&res_dropdown_clone);
        let mut selected = res_res_list_clone.borrow_mut();
        if selected.contains(&res_type.clone()) || res_vuln_list_clone.borrow().contains(&res_type.clone()) || res_imun_list_clone.borrow().contains(&res_type.clone()) {return}; // do nothing
        // 3. add res to monster
        selected.push(res_type.clone());
        selected.sort();


        // 4. add new label button combo
        let res_label = Label::builder()
            .label(String::from(res_type.clone() + ": (Resistance)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&res_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = res_flow_box_clone.clone();
        let res_list_double_clone = res_res_list_clone.clone();
        let res_clone = res_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut res_list = res_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = res_list.binary_search(&res_clone);
                if indx.is_ok(){
                    res_list.remove(indx.unwrap());
                }
            }
        });


        res_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    let imun_dropdown_clone = res_dropdown.clone();
    let imun_flow_box_clone = flow_box.clone();
    let imun_no_res_label_clone = no_res_label.clone();
    let imun_vuln_list_clone= selected_vulns.clone();
    let imun_res_list_clone= selected_res.clone();
    let imun_imun_list_clone= selected_imun.clone();
    let imun_no_res_options_clone = no_res_options.clone();
    immun_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if imun_no_res_options_clone.get() {
            imun_flow_box_clone.remove(&imun_no_res_label_clone);
            imun_no_res_options_clone.set(false)
        }

        // 2 check to see if imun already there
        let imun_type = get_dropdown_text(&imun_dropdown_clone);
        let mut selected = imun_imun_list_clone.borrow_mut();
        if selected.contains(&imun_type.clone()) || imun_vuln_list_clone.borrow().contains(&imun_type.clone()) || imun_res_list_clone.borrow().contains(&imun_type.clone()) {return}; // do nothing
        // 3. add imun to monster
        selected.push(imun_type.clone());
        selected.sort();


        // 4. add new label button combo
        let imun_label = Label::builder()
            .label(String::from(imun_type.clone() + ": (Immune)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&imun_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = imun_flow_box_clone.clone();
        let imun_list_double_clone = imun_imun_list_clone.clone();
        let imun_clone = imun_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut imun_list = imun_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = imun_list.binary_search(&imun_clone);
                if indx.is_ok(){
                    imun_list.remove(indx.unwrap());
                }
            }
        });


        imun_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    // clone every single input stream
    let monster_name_entry_clone = monster_name_entry.clone();
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let exp_entry_clone = exp_entry.clone();
    let pb_entry_clone = pb_entry.clone();
    let die_dropdown_clone = die_dropdown.clone();
    // mods
    let str_entry_clone = str_entry.clone();
    let dex_entry_clone = dex_entry.clone();
    let con_entry_clone = con_entry.clone();
    let int_entry_clone = int_entry.clone();
    let wis_entry_clone = wis_entry.clone();
    let cha_entry_clone = cha_entry.clone();
    // checkbox
    let str_check_clone = str_check.clone();
    let dex_check_clone = dex_check.clone();
    let con_check_clone = con_check.clone();
    let int_check_clone = int_check.clone();
    let wis_check_clone = wis_check.clone();
    let cha_check_clone = cha_check.clone();
    // vuln lists
    let selected_vulns_clone = selected_vulns.clone();
    let selected_res_clone = selected_res.clone();
    let selected_imun_clone = selected_imun.clone();
    // abil textbox
    let abil_entry_clone = abil_entry.clone();

    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    make_monster_button.connect_clicked(move |_| {
        // Helper function to safely parse integer values from Entry widgets.
        let parse_int_entry = |entry: &Entry, field_name: &str| -> i32 {
            match entry.text()
                .parse::<i32>()
                .map_err(|_| format!("'{}' must be a valid number.", field_name)) {
                    Ok(val) => return val,
                    Err(e) => return 0
                }
        };
        
        let name = monster_name_entry_clone.text().to_string();
        if name.trim().is_empty() { return; }

        let hp = parse_int_entry(&hp_entry_clone, "HP");
        let ac = parse_int_entry(&ac_entry_clone, "AC");        
        let exp = parse_int_entry(&exp_entry_clone, "EXP");
        let pb = parse_int_entry(&pb_entry_clone, "Proficiency Bonus");
        let str_mod = parse_int_entry(&str_entry_clone, "Strength Mod");
        let dex_mod = parse_int_entry(&dex_entry_clone, "Dexterity Mod");
        let con_mod = parse_int_entry(&con_entry_clone, "Constitution Mod");
        let int_mod = parse_int_entry(&int_entry_clone, "Intelligence Mod");
        let wis_mod = parse_int_entry(&wis_entry_clone, "Wisdom Mod");
        let cha_mod = parse_int_entry(&cha_entry_clone, "Charisma Mod");
        let mods: [i32; 6] = [str_mod,dex_mod, con_mod, int_mod, wis_mod, cha_mod];

        let hitdie = get_dropdown_text(&die_dropdown_clone);
        let str_save = str_check_clone.is_active();
        let dex_save = dex_check_clone.is_active();
        let con_save = con_check_clone.is_active();
        let int_save = int_check_clone.is_active();
        let wis_save = wis_check_clone.is_active();
        let cha_save = cha_check_clone.is_active();

        let saves = [str_save, dex_save, con_save, int_save, wis_save, cha_save];
        let abilities = abil_entry_clone.buffer().text(&abil_entry_clone.buffer().start_iter(), &abil_entry_clone.buffer().end_iter(), false);

        let new_monster = monster_manager::Monster {
            name,
            hp,
            ac,
            exp,
            pb,
            hitdie,
            mods,
            saves,
            vulnerabilities: selected_vulns_clone.borrow().clone(),
            restistances: selected_res_clone.borrow().clone(),
            immunities: selected_imun_clone.borrow().clone(),
            abilities: abilities.to_string(),
            attacks: Vec::new(),
        };

        // Save the monster and handle potential errors.
        if let Err(e) = monster_manager::save_monster(new_monster) {
            println!("Failed to save monster: {}", e);
            return;
        }

        // Close the modal and refresh the parent window's monster list.
        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    window.set_content(Some(&big_vbox));
    window.present();
}



pub fn edit_monster_creation_menu(app: &AdwApplication, parent_window: &AdwWindow, monster: Monster) {
    let window = AdwWindow::builder()
        .application(app)
        .title("Monster Builder")
        .transient_for(parent_window)
        .default_width(800)
        //.default_height(350)
        .modal(true)
        .build();

    // Main vbox
    let big_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    let title_label = Label::builder()
        .label("Create a Monster")
        .halign(Align::Center)
        .build();
    title_label.add_css_class("title-1");

    

    // top grid section; name, hp, ac, exp, pb, hit die
    let top_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();
    // get stuff for grids
    let (monster_name_label, monster_name_entry) = create_text_label_entry_pair("Monster Name:", &format!("{}", monster.name));
    let (hp_label, hp_entry) = create_text_label_entry_pair("HP:", &format!("{}", monster.hp));
    let (ac_label, ac_entry) = create_text_label_entry_pair("AC:", &format!("{}", monster.ac));
    let (exp_label, exp_entry) = create_text_label_entry_pair("EXP:", &format!("{}", monster.exp));
    let (pb_label, pb_entry) = create_text_label_entry_pair("PB:", &format!("{}", monster.pb));
    let dice_list = ["d4","d6","d8","d10","d12","d20"];
    let location = dice_list.iter().position(|x| x==(&monster.hitdie.as_str()));
    let (die_label, die_dropdown) = create_set_label_dropdown_pair("Die:", &dice_list.clone(), location.unwrap() as u32);


    // make the grid. 3 col 2 rows
    // top row
    top_grid.attach(&monster_name_label, 0, 0, 1, 1);
    top_grid.attach(&monster_name_entry, 1, 0, 1, 1);
    top_grid.attach(&hp_label, 2, 0, 1, 1);
    top_grid.attach(&hp_entry, 3, 0, 1, 1);
    top_grid.attach(&ac_label, 4, 0, 1, 1);
    top_grid.attach(&ac_entry, 5, 0, 1, 1);
    // bottom row
    top_grid.attach(&exp_label, 0, 1, 1, 1);
    top_grid.attach(&exp_entry, 1, 1, 1, 1);
    top_grid.attach(&pb_label, 2, 1, 1, 1);
    top_grid.attach(&pb_entry, 3, 1, 1, 1);
    top_grid.attach(&die_label, 4, 1, 1, 1);
    top_grid.attach(&die_dropdown, 5, 1, 1, 1);

    
    // split the lower section into 2 with an hbox
    let lower_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();
    

    // now build mod box
    let left_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();

    let mod_grid = Grid::builder()
        .row_spacing(12)
        .column_spacing(12)
        .halign(Align::Center)
        .build();
    // labels
    let mod_label = Label::builder().label("Mods").build();
    let save_label = Label::builder().label("Save Prof").build();
    // modifiers
    let (str_label, str_entry, str_check) = create_set_label_entry_checkbox_triplet("Str:", &format!("{}", monster.mods[0]), monster.saves[0]);
    let (dex_label, dex_entry, dex_check) = create_set_label_entry_checkbox_triplet("Dex:", &format!("{}", monster.mods[1]), monster.saves[1]);
    let (con_label, con_entry, con_check) = create_set_label_entry_checkbox_triplet("Con:", &format!("{}", monster.mods[2]), monster.saves[2]);
    let (int_label, int_entry, int_check) = create_set_label_entry_checkbox_triplet("Int:", &format!("{}", monster.mods[3]), monster.saves[3]);
    let (wis_label, wis_entry, wis_check) = create_set_label_entry_checkbox_triplet("Wis:", &format!("{}", monster.mods[4]), monster.saves[4]);
    let (cha_label, cha_entry, cha_check) = create_set_label_entry_checkbox_triplet("Cha:", &format!("{}", monster.mods[5]), monster.saves[5]);

    // append to grid col 3 row 7

    // col 1
    mod_grid.attach(&str_label, 0, 1, 1, 1);
    mod_grid.attach(&dex_label, 0, 2, 1, 1);
    mod_grid.attach(&con_label, 0, 3, 1, 1);
    mod_grid.attach(&int_label, 0, 4, 1, 1);
    mod_grid.attach(&wis_label, 0, 5, 1, 1);
    mod_grid.attach(&cha_label, 0, 6, 1, 1);
    // col 2
    mod_grid.attach(&mod_label, 1, 0, 1, 1);
    mod_grid.attach(&str_entry, 1, 1, 1, 1);
    mod_grid.attach(&dex_entry, 1, 2, 1, 1);
    mod_grid.attach(&con_entry, 1, 3, 1, 1);
    mod_grid.attach(&int_entry, 1, 4, 1, 1);
    mod_grid.attach(&wis_entry, 1, 5, 1, 1);
    mod_grid.attach(&cha_entry, 1, 6, 1, 1);
    // col 3
    mod_grid.attach(&save_label, 2, 0, 1, 1);
    mod_grid.attach(&str_check, 2, 1, 1, 1);
    mod_grid.attach(&dex_check, 2, 2, 1, 1);
    mod_grid.attach(&con_check, 2, 3, 1, 1);
    mod_grid.attach(&int_check, 2, 4, 1, 1);
    mod_grid.attach(&wis_check, 2, 5, 1, 1);
    mod_grid.attach(&cha_check, 2, 6, 1, 1);

    let make_monster_button = Button::builder()
        .label("Edit Monster")
        .build();
    let cancel_button = Button::builder()
        .label("Cancel")
        .halign(Align::Center)
        .build();
    let save_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    save_hbox.append(&make_monster_button);
    save_hbox.append(&cancel_button);
    
    left_vbox.append(&mod_grid);
    left_vbox.append(&save_hbox);
    

    // right side v box
    let right_vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .width_request(475)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .build();
    
    let res_label = Label::builder()
        .label("Resistances")
        .halign(Align::Start)
        .build();
    res_label.add_css_class("title-3");

    let no_res_options = Rc::new(Cell::new(true));
    let selected_vulns: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(monster.vulnerabilities));
    let selected_res: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(monster.restistances));
    let selected_imun: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(monster.immunities));
    let res_options = ["Acid","Bludgening","Cold","Fire","Force","Lightning","Necrotic", "Piercing", "Poison", "Psychic", "Radiant", "Slashing", "Thunder"];
    let res_dropdown = DropDown::builder()
        .model(&StringList::new(&res_options))
        .width_request(30)
        .margin_end(170)
        .build();

    let button_hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    let vuln_button = Button::builder()
        .halign(Align::Start)
        .label("Vulnerable")
        .build();
    let res_button = Button::builder()
        .halign(Align::Start)
        .label("Resistant")
        .build();
    let immun_button = Button::builder()
        .halign(Align::Start)
        .label("Immune")
        .build();

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

    let no_res_label = Label::builder()
        .label("No Resistances")
        .halign(Align::Start)
        .build();

    if selected_vulns.borrow().len() + selected_res.borrow().len() + selected_imun.borrow().len() == 0 {
        // No vuln or resestances. proceed as normal.
        flow_box.insert(&no_res_label, -1);
    }else {
        // add in the pre exsisting stuff
        let vuln_vuln_list_clone = selected_vulns.clone();
        let vuln_flow_box_clone = flow_box.clone();
        let itterated_copy = selected_vulns.borrow().clone();
        for vuln in itterated_copy{

            // 4. add new label button combo
            let vuln_label = Label::builder()
                .label(String::from(vuln.clone() + ": (Vulnerable)"))
                .halign(Align::Start)
                .build();
            let remove_button = Button::builder()
                .halign(Align::End)
                .label("x")
                .width_request(5)
                .build();
            let surrounding_hbox = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .build();
            surrounding_hbox.append(&vuln_label);
            surrounding_hbox.append(&remove_button);


            let surround_clone = surrounding_hbox.clone();
            let flow_box_double_clone = vuln_flow_box_clone.clone();
            let vuln_list_double_clone = vuln_vuln_list_clone.clone();
            let vuln_clone = vuln.clone();
            remove_button.connect_clicked(move |_| {
                // remove itself from the flowbox
                let mut vuln_list = vuln_list_double_clone.borrow_mut();
                if let Some(flowbox_child) = surround_clone.parent() {
                    flow_box_double_clone.remove(&flowbox_child);
                    let indx = vuln_list.binary_search(&vuln_clone);
                    if indx.is_ok(){
                        vuln_list.remove(indx.unwrap());
                    }
                }
            });
            vuln_flow_box_clone.insert(&surrounding_hbox, -1);
        }

        let res_res_list_clone = selected_res.clone();
        let res_flow_box_clone = flow_box.clone();
        let itterated_copy = selected_res.borrow().clone();
        for res in itterated_copy{

            // 4. add new label button combo
            let res_label = Label::builder()
                .label(String::from(res.clone() + ": (Resistant)"))
                .halign(Align::Start)
                .build();
            let remove_button = Button::builder()
                .halign(Align::End)
                .label("x")
                .width_request(5)
                .build();
            let surrounding_hbox = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .build();
            surrounding_hbox.append(&res_label);
            surrounding_hbox.append(&remove_button);


            let surround_clone = surrounding_hbox.clone();
            let flow_box_double_clone = res_flow_box_clone.clone();
            let res_list_double_clone = res_res_list_clone.clone();
            let res_clone = res.clone();
            remove_button.connect_clicked(move |_| {
                // remove itself from the flowbox
                let mut res_list = res_list_double_clone.borrow_mut();
                if let Some(flowbox_child) = surround_clone.parent() {
                    flow_box_double_clone.remove(&flowbox_child);
                    let indx = res_list.binary_search(&res_clone);
                    if indx.is_ok(){
                        res_list.remove(indx.unwrap());
                    }
                }
            });
            res_flow_box_clone.insert(&surrounding_hbox, -1);           
        }


        let imun_imun_list_clone = selected_imun.clone();
        let imun_flow_box_clone = flow_box.clone();
        let itterated_copy = selected_imun.borrow().clone();
        for imun in itterated_copy{
            let imun_label = Label::builder()
                .label(String::from(imun.clone() + ": (imunerable)"))
                .halign(Align::Start)
                .build();
            let remove_button = Button::builder()
                .halign(Align::End)
                .label("x")
                .width_request(5)
                .build();
            let surrounding_hbox = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .build();
            surrounding_hbox.append(&imun_label);
            surrounding_hbox.append(&remove_button);


            let surround_clone = surrounding_hbox.clone();
            let flow_box_double_clone = imun_flow_box_clone.clone();
            let imun_list_double_clone = imun_imun_list_clone.clone();
            let imun_clone = imun.clone();
            remove_button.connect_clicked(move |_| {
                // remove itself from the flowbox
                let mut imun_list = imun_list_double_clone.borrow_mut();
                if let Some(flowbox_child) = surround_clone.parent() {
                    flow_box_double_clone.remove(&flowbox_child);
                    let indx = imun_list.binary_search(&imun_clone);
                    if indx.is_ok(){
                        imun_list.remove(indx.unwrap());
                    }
                }
            });
            imun_flow_box_clone.insert(&surrounding_hbox, -1);
        }
    }

    let abil_label = Label::builder()
        .label("Abilities")
        .halign(Align::Start)
        .build();
    abil_label.add_css_class("title-3");

    let abil_entry = TextView::builder()
        .editable(true)
        .focusable(true)
        .accepts_tab(true)
        .hexpand(true)
        .vexpand(false)
        .build();
    abil_entry.add_css_class("view");
    abil_entry.buffer().set_text(&monster.abilities);

    let scrolled_container = ScrolledWindow::builder()
        .height_request(150)
        .width_request(300) // Highly recommended so it doesn't squish horizontally
        .child(&abil_entry)
        .has_frame(true) // Enforces the border frame
        .build();

    scrolled_container.add_css_class("frame");

    right_vbox.append(&res_label);
    right_vbox.append(&res_dropdown);
    right_vbox.append(&button_hbox);
    right_vbox.append(&flow_box);
    right_vbox.append(&abil_label);
    right_vbox.append(&scrolled_container);

    // build lower hbox
    lower_hbox.append(&left_vbox);
    lower_hbox.append(&right_vbox);

    // build big vbox
    big_vbox.append(&title_label);
    big_vbox.append(&top_grid);
    big_vbox.append(&lower_hbox);


    // add funcrionality the buttons

    let vuln_dropdown_clone = res_dropdown.clone();
    let vuln_flow_box_clone = flow_box.clone();
    let vuln_no_res_label_clone = no_res_label.clone();
    let vuln_vuln_list_clone= selected_vulns.clone();
    let vuln_res_list_clone= selected_res.clone();
    let vuln_imun_list_clone= selected_imun.clone();
    let vuln_no_res_options_clone = no_res_options.clone();
    vuln_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if vuln_no_res_options_clone.get() {
            vuln_flow_box_clone.remove(&vuln_no_res_label_clone);
            vuln_no_res_options_clone.set(false)
        }

        // 2 check to see if vuln already there
        let vuln_type = get_dropdown_text(&vuln_dropdown_clone);
        let mut selected = vuln_vuln_list_clone.borrow_mut();
        if selected.contains(&vuln_type.clone()) || vuln_res_list_clone.borrow().contains(&vuln_type.clone()) || vuln_imun_list_clone.borrow().contains(&vuln_type.clone()) {return}; // do nothing
        // 3. add vuln to monster
        selected.push(vuln_type.clone());
        selected.sort();

        // 4. add new label button combo
        let vuln_label = Label::builder()
            .label(String::from(vuln_type.clone() + ": (Vulnerable)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&vuln_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = vuln_flow_box_clone.clone();
        let vuln_list_double_clone = vuln_vuln_list_clone.clone();
        let vuln_clone = vuln_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut vuln_list = vuln_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = vuln_list.binary_search(&vuln_clone);
                if indx.is_ok(){
                    vuln_list.remove(indx.unwrap());
                }
            }
        });


        vuln_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    let res_dropdown_clone = res_dropdown.clone();
    let res_flow_box_clone = flow_box.clone();
    let res_no_res_label_clone = no_res_label.clone();
    let res_vuln_list_clone= selected_vulns.clone();
    let res_res_list_clone= selected_res.clone();
    let res_imun_list_clone= selected_imun.clone();
    let res_no_res_options_clone = no_res_options.clone();
    res_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if res_no_res_options_clone.get() {
            res_flow_box_clone.remove(&res_no_res_label_clone);
            res_no_res_options_clone.set(false)
        }

        // 2 check to see if res already there
        let res_type = get_dropdown_text(&res_dropdown_clone);
        let mut selected = res_res_list_clone.borrow_mut();
        if selected.contains(&res_type.clone()) || res_vuln_list_clone.borrow().contains(&res_type.clone()) || res_imun_list_clone.borrow().contains(&res_type.clone()) {return}; // do nothing
        // 3. add res to monster
        selected.push(res_type.clone());
        selected.sort();


        // 4. add new label button combo
        let res_label = Label::builder()
            .label(String::from(res_type.clone() + ": (Resistance)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&res_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = res_flow_box_clone.clone();
        let res_list_double_clone = res_res_list_clone.clone();
        let res_clone = res_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut res_list = res_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = res_list.binary_search(&res_clone);
                if indx.is_ok(){
                    res_list.remove(indx.unwrap());
                }
            }
        });


        res_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    let imun_dropdown_clone = res_dropdown.clone();
    let imun_flow_box_clone = flow_box.clone();
    let imun_no_res_label_clone = no_res_label.clone();
    let imun_vuln_list_clone= selected_vulns.clone();
    let imun_res_list_clone= selected_res.clone();
    let imun_imun_list_clone= selected_imun.clone();
    let imun_no_res_options_clone = no_res_options.clone();
    immun_button.connect_clicked(move |_| {

        // 1. detect if this is the first thing
        if imun_no_res_options_clone.get() {
            imun_flow_box_clone.remove(&imun_no_res_label_clone);
            imun_no_res_options_clone.set(false)
        }

        // 2 check to see if imun already there
        let imun_type = get_dropdown_text(&imun_dropdown_clone);
        let mut selected = imun_imun_list_clone.borrow_mut();
        if selected.contains(&imun_type.clone()) || imun_vuln_list_clone.borrow().contains(&imun_type.clone()) || imun_res_list_clone.borrow().contains(&imun_type.clone()) {return}; // do nothing
        // 3. add imun to monster
        selected.push(imun_type.clone());
        selected.sort();


        // 4. add new label button combo
        let imun_label = Label::builder()
            .label(String::from(imun_type.clone() + ": (Immune)"))
            .halign(Align::Start)
            .build();
        let remove_button = Button::builder()
            .halign(Align::End)
            .label("x")
            .width_request(5)
            .build();
        let surrounding_hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .build();
        surrounding_hbox.append(&imun_label);
        surrounding_hbox.append(&remove_button);


        let surround_clone = surrounding_hbox.clone();
        let flow_box_double_clone = imun_flow_box_clone.clone();
        let imun_list_double_clone = imun_imun_list_clone.clone();
        let imun_clone = imun_type.clone();
        remove_button.connect_clicked(move |_| {
            // remove itself from the flowbox
            let mut imun_list = imun_list_double_clone.borrow_mut();
            if let Some(flowbox_child) = surround_clone.parent() {
                flow_box_double_clone.remove(&flowbox_child);
                let indx = imun_list.binary_search(&imun_clone);
                if indx.is_ok(){
                    imun_list.remove(indx.unwrap());
                }
            }
        });


        imun_flow_box_clone.insert(&surrounding_hbox, -1);
    });

    // clone every single input stream
    let monster_name_entry_clone = monster_name_entry.clone();
    let hp_entry_clone = hp_entry.clone();
    let ac_entry_clone = ac_entry.clone();
    let exp_entry_clone = exp_entry.clone();
    let pb_entry_clone = pb_entry.clone();
    let die_dropdown_clone = die_dropdown.clone();
    // mods
    let str_entry_clone = str_entry.clone();
    let dex_entry_clone = dex_entry.clone();
    let con_entry_clone = con_entry.clone();
    let int_entry_clone = int_entry.clone();
    let wis_entry_clone = wis_entry.clone();
    let cha_entry_clone = cha_entry.clone();
    // checkbox
    let str_check_clone = str_check.clone();
    let dex_check_clone = dex_check.clone();
    let con_check_clone = con_check.clone();
    let int_check_clone = int_check.clone();
    let wis_check_clone = wis_check.clone();
    let cha_check_clone = cha_check.clone();
    // vuln lists
    let selected_vulns_clone = selected_vulns.clone();
    let selected_res_clone = selected_res.clone();
    let selected_imun_clone = selected_imun.clone();
    // abil textbox
    let abil_entry_clone = abil_entry.clone();

    let window_clone = window.clone();
    let parent_window_clone = parent_window.clone();
    let app_clone = app.clone();
    make_monster_button.connect_clicked(move |_| {
        // Helper function to safely parse integer values from Entry widgets.
        let parse_int_entry = |entry: &Entry, field_name: &str| -> i32 {
            match entry.text()
                .parse::<i32>()
                .map_err(|_| format!("'{}' must be a valid number.", field_name)) {
                    Ok(val) => return val,
                    Err(e) => return 0
                }
        };
        
        let name = monster_name_entry_clone.text().to_string();
        if name.trim().is_empty() { return; }

        let hp = parse_int_entry(&hp_entry_clone, "HP");
        let ac = parse_int_entry(&ac_entry_clone, "AC");        
        let exp = parse_int_entry(&exp_entry_clone, "EXP");
        let pb = parse_int_entry(&pb_entry_clone, "Proficiency Bonus");
        let str_mod = parse_int_entry(&str_entry_clone, "Strength Mod");
        let dex_mod = parse_int_entry(&dex_entry_clone, "Dexterity Mod");
        let con_mod = parse_int_entry(&con_entry_clone, "Constitution Mod");
        let int_mod = parse_int_entry(&int_entry_clone, "Intelligence Mod");
        let wis_mod = parse_int_entry(&wis_entry_clone, "Wisdom Mod");
        let cha_mod = parse_int_entry(&cha_entry_clone, "Charisma Mod");
        let mods: [i32; 6] = [str_mod,dex_mod, con_mod, int_mod, wis_mod, cha_mod];

        let hitdie = get_dropdown_text(&die_dropdown_clone);
        let str_save = str_check_clone.is_active();
        let dex_save = dex_check_clone.is_active();
        let con_save = con_check_clone.is_active();
        let int_save = int_check_clone.is_active();
        let wis_save = wis_check_clone.is_active();
        let cha_save = cha_check_clone.is_active();

        let saves = [str_save, dex_save, con_save, int_save, wis_save, cha_save];
        let abilities = abil_entry_clone.buffer().text(&abil_entry_clone.buffer().start_iter(), &abil_entry_clone.buffer().end_iter(), false);

        let new_monster = monster_manager::Monster {
            name,
            hp,
            ac,
            exp,
            pb,
            hitdie,
            mods,
            saves,
            vulnerabilities: selected_vulns_clone.borrow().clone(),
            restistances: selected_res_clone.borrow().clone(),
            immunities: selected_imun_clone.borrow().clone(),
            abilities: abilities.to_string(),
            attacks: Vec::new(),
        };

        // Save the monster and handle potential errors.
        if let Err(e) = monster_manager::save_monster(new_monster) {
            println!("Failed to save monster: {}", e);
            return;
        }

        // Close the modal and refresh the parent window's monster list.
        window_clone.close();
        switch_to_monster_list(&app_clone, &parent_window_clone);
    });

    let window_clone_cancel = window.clone();
    cancel_button.connect_clicked(move |_| {
        window_clone_cancel.close();
    });

    window.set_content(Some(&big_vbox));
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
                    monster.mods[0], monster.mods[1], monster.mods[2],
                    monster.mods[3], monster.mods[4], monster.mods[5]))
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
        .max_width_chars(15)
        .build();

    (label, entry)
}
fn create_label_entry_checkbox_triplet(label_text: &str, placeholder_text: &str) -> (Label, Entry, CheckButton) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();

    let entry = Entry::builder()
        .placeholder_text(placeholder_text)
        .max_width_chars(3)
        .build();
    
    let check = CheckButton::builder()
        .build();

    (label, entry, check)
}

fn create_set_label_entry_checkbox_triplet(label_text: &str, starting_text: &str,starting_check: bool) -> (Label, Entry, CheckButton) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();

    let entry = Entry::builder()
        .text(starting_text)
        .max_width_chars(3)
        .build();
    
    let check = CheckButton::builder()
        .active(starting_check)
        .build();

    (label, entry, check)
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


fn create_label_checkbox_pair(label_text: &str) -> (Label, CheckButton) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();

    let entry = CheckButton::new();

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
        .width_request(15)
        .build();

    (label, dropdown)
}

fn create_set_label_dropdown_pair(label_text: &str, items: &[&str], selected: u32) -> (Label, DropDown) {
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();
    
    let string_list = StringList::new(items);

    let dropdown = DropDown::builder()
        .model(&string_list)
        .selected(selected)
        .width_request(15)
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
    let (saving_throw_label, saving_throw_checkbox) = create_label_checkbox_pair("Is this a saving throw?");
    
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
    input_grid.attach(&saving_throw_label, 0, 5, 1, 1);
    input_grid.attach_next_to(&saving_throw_checkbox, Some(&saving_throw_label), gtk::PositionType::Right, 1, 1);
    
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
    let saving_throw_checkbox_clone = saving_throw_checkbox.clone();

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

pub fn get_dropdown_text(dropdown: &DropDown) -> String{
    return dropdown.selected_item()
        .and_then(|obj| obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string()))
        .unwrap_or_default()
}