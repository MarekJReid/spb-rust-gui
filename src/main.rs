mod state;
mod simulation;
use crate::state::AppState;
use crate::simulation::mqtt_handler::execute_simulation;
use std::cell::RefCell;
use std::rc::Rc;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Builder, Button, Entry, FileChooserButton, SpinButton, TextView, Window};
fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let glade_src = include_str!("spb-guiv0.1.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder
        .get_object("main_window")
        .expect("Couldn't get main_window");

    // Create an Rc<RefCell<AppState>> instance
    let app_state = Rc::new(RefCell::new(AppState::new()));
    // get the file paths for the d-messages
    let dbirth_file_chooser_button: FileChooserButton = builder
        .get_object("dbirth_file_chooser_button")
        .expect("Couldn't get dbirth_file_chooser_button");
    let app_state_clone = app_state.clone();
    dbirth_file_chooser_button.connect_file_set(move |fcd_button| {
        if let Some(path) = fcd_button.get_filename() {
            app_state_clone
                .borrow_mut()
                .set_dbirth_file_path(path.to_string_lossy().into_owned());
            println!("Selected file path: {:?}", path);
            // Save the path or file reference as needed
        }
    });
    let ddata_file_chooser_button: FileChooserButton = builder
        .get_object("ddata_file_chooser_button")
        .expect("Couldn't get ddata_file_chooser_button");
    let app_state_clone = app_state.clone();
    ddata_file_chooser_button.connect_file_set(move |fc_button| {
        if let Some(path) = fc_button.get_filename() {
            app_state_clone
                .borrow_mut()
                .set_ddata_file_path(path.to_string_lossy().into_owned());
            // Save the path or file reference as needed
        }
    });
    let ddeath_file_chooser_button: FileChooserButton = builder
        .get_object("ddeath_file_chooser_button")
        .expect("Couldn't get ddata_file_chooser_button");
    let app_state_clone = app_state.clone();
    ddeath_file_chooser_button.connect_file_set(move |fc_button| {
        if let Some(path) = fc_button.get_filename() {
            app_state_clone
                .borrow_mut()
                .set_ddeath_file_path(path.to_string_lossy().into_owned());
        }
    });

    // Get the value from teh iterator to set a number of devices to simulate

    let spin_button: SpinButton = builder
        .get_object("number_devices_to_simulate")
        .expect("Couldnt get value of spin button");
    let app_state_clone = app_state.clone();
    spin_button.connect_value_changed(move |sb| {
        let value = sb.get_value();
        app_state_clone
            .borrow_mut()
            .set_number_devices_to_simulate(value);
        println!("SpinButton value changed: {}", value);
        // Additional actions with the value
    });

    let mqtt_broker_hostname: Entry = builder
        .get_object("mqtt_broker_hostname")
        .expect("Couldnget get mqtt hostname");

    let app_state_clone = app_state.clone();
    mqtt_broker_hostname.connect_changed(move |e| {
        let text = e.get_text();
        app_state_clone
            .borrow_mut()
            .set_mqtt_broker_hostname(text.to_string());
    });

    let simulate_button: Button = builder
        .get_object("simulate_button")
        .expect("Couldnt get button obect");

    let text_view: TextView = builder
        .get_object("text_view")
        .expect("Couldn't get text_view");
    simulate_button.connect_clicked(move |_| {
        // run_simulation(&app_state, &text_view);
        execute_simulation(&app_state, &text_view);
        // Perform any additional actions with the text here
    });

    window.show_all();
    gtk::main();
}
