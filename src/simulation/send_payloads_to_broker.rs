use rumqttc::{Client, MqttOptions, QoS, Incoming, Event};
use serde_json::Value;
use std::{cell::RefCell, error::Error, rc::Rc, thread, time::Duration};
use crate::AppState;
pub fn send_payloads_to_broker(
    app_state: &Rc<RefCell<AppState>>,
    payloads: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let app_state_borrow = app_state.borrow();
    let hostname = app_state_borrow.mqtt_broker_hostname();

    let interval: u64 = 1000; // Duration in milliseconds
    let broker_identifier = "kifabs";
    let topic_name = "spb-cli-1703087130";

    let mut mqttoptions = MqttOptions::new(broker_identifier, hostname, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);


let mut is_connected = false; // Initialize the flag as false

let _handle = std::thread::spawn(move || {
    for notification in connection.iter() {
        if let Ok(Event::Incoming(Incoming::ConnAck(_))) = notification {
            if !is_connected {
                println!("Connection with the broker has been established");
                is_connected = true; // Set the flag to true after the first connection
            } else {
                println!("Already connected to the broker");
            }
        }
    }
});


client.subscribe(topic_name, QoS::AtMostOnce)?;
for payload in payloads {
    println!("Sending message: {}", &payload);
    client.publish(&*topic_name, QoS::AtLeastOnce, false, &*payload)?;
    thread::sleep(Duration::from_millis(interval));
}

    Ok(())
}
