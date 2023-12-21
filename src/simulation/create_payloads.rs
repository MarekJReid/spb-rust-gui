use serde_json::Value;
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
    let numMessagesToSend: u32 = 30;
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
    if let serde_json::Value::Array(metrics) = metrics {
        for metric in metrics {
          let serialized_metric = serde_json::to_string(metric).unwrap_or_else(|_| "Invalid JSON".to_string());
          serialized_metrics.push(serialized_metric);
        }
    }
    // print the contents for now
    // println!("I am the json file contents: {:?}", metrics );

    Ok(serialized_metrics)
}


fn createDDataMessages(ddata_file_path: String, numMessages: u32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(ddata_file_path)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // pass the json
    let jsonFromFile: Value = serde_json::from_str(&contents)?;

    let metrics = &jsonFromFile["metrics"];

    let mut serialized_metrics = Vec::new();
    if let serde_json::Value::Array(metrics) = metrics {
        for metric in metrics {
          let serialized_metric = serde_json::to_string(metric).unwrap_or_else(|_| "Invalid JSON".to_string());
          serialized_metrics.push(serialized_metric);
        }
    }
    // now we create the amount of extra messages with the numMessages variable
    // using the first metric in the serialized_metrics as a tempalte
    //
  

if !serialized_metrics.is_empty() {
    let mut additional_metrics = Vec::new();

    for _ in 0..numMessages {
        for metric_str in &serialized_metrics {
            let mut metric: serde_json::Value = serde_json::from_str(metric_str)?;

            if let Some(value) = metric.get_mut("value") {
               let random_value = rand::thread_rng().gen_range(-10.0..=150.0);
*value = serde_json::Value::Number(serde_json::Number::from_f64(random_value).unwrap());
            }
            // Update other fields as necessary, like timestamp

            let new_serialized_metric = serde_json::to_string(&metric).unwrap_or_else(|_| "Invalid JSON".to_string());
            additional_metrics.push(new_serialized_metric);
        }
    }

    serialized_metrics.extend(additional_metrics);
}

// Now `serialized_metrics` contains the original and additional metrics


      // print the contents for now
    // println!("I am the json file contents: {:?}", metrics );

    Ok(serialized_metrics)
}
