use gtk::{ prelude::* };
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;

// import local script
mod monster_manager;
mod interface;
mod simulation;
mod ui_factory;

const APP_ID: &str = "github.com/Error401-UNF/Mass-Combat-Decider";

fn main() {
    // Create a new application
    let app = AdwApplication::builder() // Use AdwApplication
        .application_id(APP_ID)
        .build();
    let _ = gtk::init();
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        .bloodied {
            /*background-color: rgba(239, 68, 68, 0.25);  Faint red background */
            border: 1px solid rgb(220, 38, 38);        /* Striking red border */
        }
    "
    );
    // Use the gdk module re-exported inside the gtk crate
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    // Check for monsters
    if !monster_manager::check_for_monsters() {
        // monsters do not exsist. build first start ui
        app.connect_activate(first_start);
    } else {
        app.connect_activate(monster_list);
        println!("monster list active from boot");
    }

    // Run the application
    app.run();
}

fn first_start(app: &AdwApplication) {
    // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .modal(true)
        .build();
    interface::switch_to_first_time(&app, &window);
}

fn monster_list(app: &AdwApplication) {
    // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .modal(true)
        .build();
    interface::switch_to_monster_list(&app, &window);
}
