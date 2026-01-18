use gtk::{prelude::*};
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;

// import local script
mod monster_manager;
mod ui_manager;
mod simulation;


const APP_ID: &str = "org.example.GtkRsApp";

fn main() {
    // Create a new application
    let app = AdwApplication::builder() // Use AdwApplication
        .application_id(APP_ID)
        .build();
    
    // Check for monsters
    if !monster_manager::check_for_monsters() {
        // monsters do not exsist. build first start ui
        app.connect_activate(first_start);
    }
    else {
        app.connect_activate(monster_list);
        println!("monster list active from boot")
    }

    // Run the application
    app.run();
}

fn first_start(app: &AdwApplication) { // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .modal(true)
        .build();
    ui_manager::switch_to_first_time(&app, &window);
}



fn monster_list(app: &AdwApplication) { // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .modal(true)
        .build();
    ui_manager::switch_to_monster_list(&app, &window);
}