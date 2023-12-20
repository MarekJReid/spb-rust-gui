use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cell::RefCell;
use crate::AppState;
use std::error::Error;
use crate::Rc;
pub fn create_payloads(app_state: &Rc<RefCell<AppState>>) -> Result<(), Box<dyn Error>> {
    let app_state_borrow = app_state.borrow();
    let dbirth_file_path = app_state_borrow.dbirth_file_path();
    let ddata_file_path = app_state_borrow.ddata_file_path();
    let ddeath_file_path = app_state_borrow.ddeath_file_path();
println!("#######i am the path {}", dbirth_file_path);
createDbirthMessages(dbirth_file_path.to_string());
    Ok(())
}

fn createDbirthMessages(dbirth_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(dbirth_file_path)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    // pass the json
    let json: Value = serde_json::from_str(&contents)?;

    // print the contents for now
    println!("I am the json file contents: {:?}", json );

    Ok(())
}