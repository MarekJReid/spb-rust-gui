// use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::fs::File;
// use std::time::Duration;
// use std::{process, thread};
//
// use std::io::Read;
// use std::rc::Rc;
//
// use gtk::{TextBufferExt, TextView, TextViewExt};
// use rumqttc::{Client, MqttOptions, QoS};
//
// use serde_json;
//
// use crate::state::AppState;
//
// pub fn execute_simulation(app_state: &Rc<RefCell<AppState>>, text_view: &TextView) {
//     // Get the values out of state to dervice broke query
//     //
//
//     // Create a client & define connect options
//     let mut mqttoptions = MqttOptions::new("NAME", "localhost", 1883);
//     mqttoptions.set_keep_alive(Duration::from_secs(5));
//     let (mut client, mut connection) = Client::new(mqttoptions, 10);
//     client.subscribe("kifabs", QoS::AtMostOnce).unwrap();
//     thread::spawn(move || {
//         for i in 0..10 {
//             client
//                 .publish("mksbox1v1", QoS::AtLeastOnce, false, payload)
//                 .unwrap();
//             thread::sleep(Duration::from_millis(1000));
//         }
//     });
//     for (_i, message) in connection.iter().enumerate() {
//         println!("Message= {:?}", message);
//     }
// }
//
// // Define a struct to represent the metrics in the JSON file
// #[derive(Debug, Deserialize, Serialize)]
// struct Metric {
//     name: String,
//     alias: u32,
//     timestamp: u64,
//     datatype: u32,
//     value: String,
//     // Add other fields as needed
// }
// // Define a struct to represent the JSON data
// #[derive(Debug, Deserialize, Serialize)]
// struct JsonData {
//     timestamp: u64,
//     metrics: Vec<Metric>,
//     seq: u32,
// }
//
// fn generate_birth_metrics(
//     dbirth_file_path: &str,
// ) -> Result<Vec<Metric>, Box<dyn std::error::Error>> {
//     // Read the JSON file
//     let mut file = File::open(dbirth_file_path)?;
//     let mut json_str = String::new();
//     file.read_to_string(&mut json_str)?;
//
//     // Parse the JSON data into a Vec<Metric>
//     let json_data: JsonData = serde_json::from_str(&json_str)?;
//
//     // Extract the metrics from the JSON data
//     let metrics = json_data.metrics;
//
//     Ok(metrics)
// }
//
// // Define a function to send an item to the MQTT broker
//
// // Define a function to send an item to the MQTT broker
//
// // Define a function to update the text_view with a new message
// fn update_text_view(text_view: &TextView, message: &str) {
//     let buffer = text_view.get_buffer().expect("Failed to get text buffer");
//     let mut end_iter = buffer.get_end_iter();
//
//     // Append the message to the text_view
//     buffer.insert(&mut end_iter, message);
//
//     // Add a newline after the message for separation
//     buffer.insert(&mut end_iter, "\n");
// }
