use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn create_payloads(app_state: &Rc<RefCell<AppState>>) -> Result<(), Box<dyn Error>> {
    let app_state_borrow = app_state.borrow();
    let dbirth_file_path = app_state_borrow.dbirth_file_path();
    let ddata_file_path = app_state_borrow.ddata_file_path();
    let ddeath_file_path = app_state_borrow.ddeath_file_path();

    Ok(())
}

fn createDbirthMessages(dbirth_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(dbirth_file_path)?;
    let reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // pass the json
    let json: Value = serde_json::from_str(&contents)?;

    // print the contents for now
    println!("I am the json file contents", json);
    Ok(())
}
