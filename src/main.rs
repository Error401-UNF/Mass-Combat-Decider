use gtk::{ prelude::* };
use libadwaita::Application as AdwApplication;
use libadwaita::Window as AdwWindow;
use libadwaita::prelude::AdwWindowExt;

// import local script
mod monster_manager;
mod interface;
mod simulation;
mod ui_factory;

const APP_ID: &str = "github.com/Error401-UNF/Mass-Combat-Decider";

fn main() {
    if std::env::var("GTK_THEME").is_err() {
        unsafe { std::env::set_var("GTK_THEME", "Adwaita") };
    }
    // Create a new application
    let app = AdwApplication::builder() // Use AdwApplication
        .application_id(APP_ID)
        .build();
    let _ = gtk::init();
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        .bloodied {
            border: 1px solid rgb(220, 38, 38);
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
        .build();

    wrap_view_with_header_bar(&window, |content_box| {
        interface::switch_to_first_time(app, &window);
    });
}

fn monster_list(app: &AdwApplication) {
    // Use AdwApplication
    // Create a new window
    let window = AdwWindow::builder() // Use AdwWindow
        .application(app)
        .title("Mass Combat Decider")
        .build();
    // Wrap the interface layout inside a HeaderBar container so it can be dragged
    wrap_view_with_header_bar(&window, |content_box| {
        interface::switch_to_monster_list(app, &window);
    });
}

fn wrap_view_with_header_bar<F>(window: &AdwWindow, build_inner_ui: F)
where
    F: FnOnce(&gtk::Box),
{
    let outer_vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    
    // Create Libadwaita's native HeaderBar to receive title-grabbing/drag events
    let header_bar = libadwaita::HeaderBar::new();
    outer_vbox.append(&header_bar);

    let inner_content_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    inner_content_container.set_vexpand(true);
    inner_content_container.set_hexpand(true);
    outer_vbox.append(&inner_content_container);

    window.set_content(Some(&outer_vbox));
    
    // Switch contents using your interface routers inside the nested container
    build_inner_ui(&inner_content_container);
}
