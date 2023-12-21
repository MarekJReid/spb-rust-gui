
// Assuming 'metric.rs' is the generated file from your .proto file
// Assuming 'metric.rs' is the generated file from your .proto file
use crate::simulation::spb::Payload_Metric; // Import the Metric struct
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cell::RefCell;
use crate::AppState;
use std::error::Error;
use crate::Rc;
use rand::Rng;
pub fn create_payloads(app_state: &Rc<RefCell<AppState>>) -> Result<Vec<String>, Box<dyn Error>> {
    let app_state_borrow = app_state.borrow();
    let dbirth_file_path = app_state_borrow.dbirth_file_path();
    let ddata_file_path = app_state_borrow.ddata_file_path();
    let ddeath_file_path = app_state_borrow.ddeath_file_path();

    
let mut payloads: Vec<String> = Vec::new();

    let dbirthMessagesToPush = createDbirthMessages(dbirth_file_path.to_string());
if let Ok(messages) = dbirthMessagesToPush {
    payloads.extend(messages);
} else {
    println!("didnt work");
}
    //TODO: Here i am setting it manually but should be a user entry 
    let numMessagesToSend: u32 = 3;
    let ddataMessagesToPush = createDDataMessages(ddata_file_path.to_string(), numMessagesToSend);

if let Ok(messages) = ddataMessagesToPush {
    payloads.extend(messages);
} else {
    println!("didnt work");
}

    // println!("payloads: {:?}", payloads);
    Ok(payloads)
}

fn createDbirthMessages(dbirth_file_path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(dbirth_file_path)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // pass the json
    let jsonFromFile: Value = serde_json::from_str(&contents)?;

    let metrics = &jsonFromFile["metrics"];

    let mut serialized_metrics = Vec::new();

    if let Value::Array(metrics) = metrics {
    for metric in metrics {
        let mut metric_clone = metric.clone(); // Clone the metric to make it mutable

        // Get the current Unix timestamp
        if let Ok(since_the_epoch) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let timestamp = since_the_epoch.as_secs(); // seconds since epoch
            // You can also include milliseconds if needed:
            // let timestamp_millis = since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis());

            // Set the timestamp field
            if let Value::Object(ref mut map) = metric_clone {
                map.insert("timestamp".to_string(), Value::Number(timestamp.into()));
            }
        }

        // Serialize the metric to a JSON string
        if let Ok(serialized) = serde_json::to_string(&metric_clone) {
            serialized_metrics.push(serialized);
        } else {
            eprintln!("Failed to serialize metric");
        }
    }    // print the contents for now
    // println!("I am the json file contents: {:?}", metrics );
}
    Ok(serialized_metrics)
}


fn createDDataMessages(ddata_file_path: String, num_messages: u32) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    // Open and read the file
    let file = File::open(ddata_file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // Deserialize the JSON
    let json_from_file: Value = serde_json::from_str(&contents)?;
    let metrics = json_from_file["metrics"].as_array().ok_or("Expected an array of metrics")?;

    let mut serialized_metrics = Vec::new();

    // Process each metric
    for metric in metrics {
        let mut metric_clone = metric.clone();
        
        // Set the timestamp field
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if let Value::Object(ref mut map) = metric_clone {
            map.insert("timestamp".to_string(), json!(timestamp));
        }

        // Convert to Metric and serialize
        let metric_protobuf = json_to_metric(&metric_clone)?;
        let serialized = metric_protobuf.write_to_bytes()?;
        serialized_metrics.push(serialized);
    }

    // Create additional metrics
    let mut additional_metrics = Vec::new();
    if let Some(first_metric) = serialized_metrics.first() {
        for _ in 0..num_messages {
            let mut rng = rand::thread_rng();
            let random_value = rng.gen_range(-10.0..=150.0);

            // Assume first_metric is a serialized Metric. Deserialize it first, modify, and re-serialize
            let mut metric: Metric = Metric::parse_from_bytes(first_metric)?;
            // Update the metric value, assuming it's a float_value for this example
            metric.set_float_value(random_value as f32);

            // Re-serialize the updated metric
            let new_serialized_metric = metric.write_to_bytes()?;
            additional_metrics.push(new_serialized_metric);
        }
    }

    serialized_metrics.extend(additional_metrics);

    Ok(serialized_metrics)
}
// Now `serialized_metrics` contains the original and additional metrics


      // print the contents for now
    // println!("I am the json file contents: {:?}", metrics );



fn json_to_metric(json_metric: &Value) -> Result<Payload_Metric, Box<dyn std::error::Error>> {
    let mut metric = Payload_Metric::new();

    if let Some(alias) = json_metric["alias"].as_u64() {
        metric.set_alias(alias);
    }

    if let Some(name) = json_metric["name"].as_str() {
        metric.set_name(name.to_string());
    }

    if let Some(datatype) = json_metric["datatype"].as_u64() {
        metric.set_datatype(datatype.try_into().unwrap());
    }
    if let Some(timestamp) = json_metric["timestamp"].as_u64() {
        metric.set_datatype(timestamp.try_into().unwrap());
    }
      // Handling the 'value' oneof field
    let value_field = &json_metric["value"];
    if let Some(v) = to_u32(value_field) {
        metric.set_int_value(v);
    } else if let Some(v) = to_u64(value_field) {
        metric.set_long_value(v);
    } else if let Some(v) = to_f32(value_field) {
        metric.set_float_value(v);
    } else if let Some(v) = to_f64(value_field) {
        metric.set_double_value(v);
    } else if let Some(v) = to_bool(value_field) {
        metric.set_boolean_value(v);
    } else if let Some(v) = to_string(value_field) {
        metric.set_string_value(v);
    }  

    Ok(metric)
}

fn to_u32(value: &Value) -> Option<u32> {
    value.as_u64().map(|v| v as u32)
}

fn to_u64(value: &Value) -> Option<u64> {
    value.as_u64()
}

fn to_f32(value: &Value) -> Option<f32> {
    value.as_f64().map(|v| v as f32)
}

fn to_f64(value: &Value) -> Option<f64> {
    value.as_f64()
}

fn to_bool(value: &Value) -> Option<bool> {
    value.as_bool()
}

fn to_string(value: &Value) -> Option<String> {
    value.as_str().map(ToString::to_string)
}



