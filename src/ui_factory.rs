use gtk::{ Adjustment, Align, Box, Button, CheckButton, DropDown, Entry, Label, Orientation, ScrolledWindow, SpinButton, StringObject, glib::object::Cast, prelude::WidgetExt};

pub struct UiFactory;

impl UiFactory {
    pub fn create_box(
        orientation: Orientation,
        spacing: i32,
        margins: (i32, i32, i32, i32)
    ) -> Box {
        Box::builder()
            .orientation(orientation)
            .spacing(spacing)
            .margin_top(margins.0)
            .margin_bottom(margins.1)
            .margin_start(margins.2)
            .margin_end(margins.3)
            .build()
    }

    pub fn create_label(text: &str, halign: Align, markup: bool, css_classes: &[&str]) -> Label {
        let builder = Label::builder().label(text).halign(halign).use_markup(markup);
        let label = builder.build();
        for class in css_classes {
            label.add_css_class(class);
        }
        label
    }

    pub fn create_scrolled_window(
        vexpand: bool,
        hexpand: bool,
        height_request: Option<i32>
    ) -> ScrolledWindow {
        let mut builder = ScrolledWindow::builder().vexpand(vexpand).hexpand(hexpand);
        if let Some(height) = height_request {
            builder = builder.height_request(height);
        }
        builder.build()
    }

    pub fn create_button(label: &str, halign: Align, css_class: Option<&str>) -> Button {
        let builder = Button::builder().label(label).halign(halign);
        let button = builder.build();
        if let Some(class) = css_class {
            button.add_css_class(class);
        }
        button
    }

    pub fn create_spin_button(min: f64, max: f64, step: f64, initial: f64) -> SpinButton {
        let adj = Adjustment::new(initial, min, max, step, 5.0, 0.0);
        SpinButton::builder()
            .adjustment(&adj)
            .numeric(true)
            .focusable(false)
            .build()
    }

    pub fn create_entry(
        text: Option<&str>,
        placeholder: Option<&str>,
        max_width: i32
    ) -> gtk::Entry {
        let mut builder = gtk::Entry::builder().max_width_chars(max_width);
        if let Some(t) = text {
            builder = builder.text(t);
        }
        if let Some(p) = placeholder {
            builder = builder.placeholder_text(p);
        }
        builder.build()
    }

    pub fn create_check_button(active: bool) -> gtk::CheckButton {
        gtk::CheckButton::builder().active(active).build()
    }

    pub fn create_dropdown(
        items: &[&str],
        selected: Option<u32>,
        width_request: Option<i32>
    ) -> gtk::DropDown {
        let string_list = gtk::StringList::new(items);
        let mut builder = gtk::DropDown::builder().model(&string_list);
        if let Some(sel) = selected {
            builder = builder.selected(sel);
        }
        if let Some(wr) = width_request {
            builder = builder.width_request(wr);
        }
        builder.build()
    }

    pub fn create_grid(row_spacing: i32, col_spacing: i32, halign: Align) -> gtk::Grid {
        gtk::Grid
            ::builder()
            .row_spacing(row_spacing)
            .column_spacing(col_spacing)
            .halign(halign)
            .build()
    }

    pub fn create_label_entry_pair(label_text: &str, placeholder_text: &str) -> (Label, Entry) {
        let label = UiFactory::create_label(label_text, Align::Start, false, &[]);
        let entry = UiFactory::create_entry(None, Some(placeholder_text), 15);
        (label, entry)
    }

    pub fn create_label_checkbox_pair(label_text: &str) -> (Label, CheckButton) {
        let label = UiFactory::create_label(label_text, Align::Start, false, &[]);
        let entry = UiFactory::create_check_button(false);
        (label, entry)
    }

    pub fn create_label_dropdown_pair(label_text: &str, items: &[&str]) -> (Label, DropDown) {
        let label = UiFactory::create_label(label_text, Align::Start, false, &[]);
        let dropdown = UiFactory::create_dropdown(items, None, Some(15));
        (label, dropdown)
    }

    pub fn get_dropdown_text(dropdown: &DropDown) -> String {
        dropdown
            .selected_item()
            .and_then(|obj| {
                obj.downcast_ref::<StringObject>().map(|s_obj| s_obj.string().to_string())
            })
            .unwrap_or_default()
    }
}
