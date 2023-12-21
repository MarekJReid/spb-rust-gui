use std::cell::RefCell;
use std::rc::Rc;
use crate::state::AppState;
use gtk::TextView;
 use crate::simulation::send_payloads_to_broker::send_payloads_to_broker;  
use crate::simulation::create_payloads::create_payloads;

pub fn execute_simulation(app_state: &Rc<RefCell<AppState>>, text_view: &TextView) {
    println!("########### woot im doing osmethign ################");
    let payloads_result = create_payloads(app_state);

    match payloads_result {
        Ok(payloads) => {
            println!("payloads to push {:?}", payloads);
            // If send_payloads_to_broker expects Vec<String>
            send_payloads_to_broker(&app_state, payloads);
        }
        Err(e) => {
            // Handle the error, e.g., log it or display it
            println!("Error creating payloads: {}", e);
        }
    }
}

