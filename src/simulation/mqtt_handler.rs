use std::cell::RefCell;
use std::rc::Rc;
use crate::state::AppState;
use gtk::TextView;
 use crate::simulation::send_payloads_to_broker::send_payloads_to_broker;  
use crate::simulation::create_payloads::create_payloads;
pub fn execute_simulation(app_state: &Rc<RefCell<AppState>>, text_view: &TextView) {
    // first we are going to create the functions to create the payloads
    println!("########### woot im doing osmethign ################");
    let payloads = create_payloads(app_state);
    // send payloads to broker
   // send_payloads_to_broker(&app_state, payloads);
}
