use rumqttc::{Client, MqttOptions, QoS};
use serde_json::Value;
use std::{cell::RefCell, error::Error, rc::Rc, thread, time::Duration};

pub fn send_payloads_to_broker(
    app_state: &Rc<RefCell<AppState>>,
    payloads: Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    let app_state_borrow = app_state.borrow();
    let hostname = app_state_borrow.mqtt_broker_hostname();

    let interval: u64 = 1000; // Duration in milliseconds
    let broker_identifier = "kifabs";
    let topic_name = "device1";

    let mut mqttoptions = MqttOptions::new(broker_identifier, hostname, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, _connection) = Client::new(mqttoptions, 10);

    // client.subscribe("kifabs", QoS::AtMostOnce)?;
    // for payload in payloads {
    //     client.publish(&topic_name, QoS::AtLeastOnce, false, &payload)?;
    //     thread::sleep(Duration::from_millis(interval));
    // }
    //
    Ok(())
}
