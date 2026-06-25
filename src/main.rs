use gtk::{ prelude::* };
use libadwaita::Application as AdwApplication;
use gtk::ApplicationWindow as AdwWindow;

// import local script
mod monster_manager;
mod interface;
mod simulation;
mod ui_factory;

const APP_ID: &str = "com.mass.combat.decider";
const EMBEDDED_THEME_CSS: &str = include_str!("theme.css");

fn main() {
    // Create a new application
    let app = AdwApplication::builder() // Use AdwApplication
        .application_id(APP_ID)
        .build();
    
    let _ = gtk::init();
    let provider = gtk::CssProvider::new();
    provider.load_from_data(EMBEDDED_THEME_CSS);
    // Use the gdk module re-exported inside the gtk crate
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    // Check for monsters and activate appropriate UI
    if !monster_manager::check_for_monsters() {
        app.connect_activate(first_start);
    } else {
        app.connect_activate(monster_list);
    }

    app.run();
}

fn first_start(app: &AdwApplication) {
    // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .default_width(900)
        .default_height(800)
        .build();

    let header_bar = libadwaita::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));
    interface::switch_to_first_time(app, &window);
}

fn monster_list(app: &AdwApplication) {
    // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .default_width(900)
        .default_height(800)
        .build();

    let header_bar = libadwaita::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));
    interface::switch_to_monster_list(app, &window);
}
