pub fn execute_simulation(app_state: &Rc<RefCell<AppState>>, text_view: &TextView) {
    // first we are going to create the functions to create the payloads
    let payloads = create_payloads(&app_state);
    // send payloads to broker
    send_payloads_to_broker(&app_state, payloads);
}
