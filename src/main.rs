extern crate gtk;
use gtk::prelude::*;
use gtk::{ComboBoxText, FileChooserAction, FileChooserDialog, Label, Orientation, Scale, Window, WindowType};
use std::path::PathBuf;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Photo Compressor");
    window.set_default_size(400, 200);

    let vbox = gtk::Box::new(Orientation::Vertical, 5);

    let scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    vbox.pack_start(&scale, false, false, 5);

    let hbox = gtk::Box::new(Orientation::Horizontal, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 0);

    let combo_box = ComboBoxText::new();
    combo_box.append_text("KB");
    combo_box.append_text("MB");
    combo_box.set_active(Some(0)); // Set default to KB
    hbox.pack_start(&combo_box, false, false, 0);

    vbox.pack_start(&hbox, false, false, 5);

    let choose_button = gtk::Button::with_label("Choose File");
    let filename_label = Label::new(None);
    vbox.pack_start(&choose_button, false, false, 5);
    vbox.pack_start(&filename_label, false, false, 5);

    let save_button = gtk::Button::with_label("Save As");
    vbox.pack_start(&save_button, false, false, 5);

    window.add(&vbox);

    // Connecting signals
    let scale_clone = scale.clone();
    entry.connect_activate(move |_| {
        let value = entry.get_text().unwrap().parse::<f64>().unwrap_or(0.0);
        scale_clone.set_value(value);
    });

    let entry_clone = entry.clone();
    scale.connect_value_changed(move |scale| {
        let value = scale.get_value();
        entry_clone.set_text(&value.to_string());
    });

    choose_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Choose a file"),
            Some(&window),
            FileChooserAction::Open,
        );
        dialog.add_buttons(&[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept.into()),
        ]);

        if dialog.run() == gtk::ResponseType::Accept.into() {
            if let Some(filename) = dialog.get_filename() {
                filename_label.set_text(&format!("Selected file: {}", filename.to_string_lossy()));
            }
        }

        dialog.destroy();
    });

    save_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::new(
            Some("Save as..."),
            Some(&window),
            FileChooserAction::Save,
        );
        dialog.add_buttons(&[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept.into()),
        ]);

        if dialog.run() == gtk::ResponseType::Accept.into() {
            if let Some(filename) = dialog.get_filename() {
                let save_path = format!("{}{}", filename.to_string_lossy(), ".compressed");
                filename_label.set_text(&format!("File saved as: {}", save_path));
            }
        }

        dialog.destroy();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
