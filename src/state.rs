// state.rs

#[derive(Debug)]
pub struct AppState {
    mqtt_broker_hostname: String,
    number_devices_to_simulate: f64,
    ddeath_file_path: String,
    ddata_file_path: String,
    dbirth_file_path: String,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mqtt_broker_hostname: String::new(),
            number_devices_to_simulate: 0.0,
            ddeath_file_path: String::new(),
            ddata_file_path: String::new(),
            dbirth_file_path: String::new(),
        }
    }

    // Getters
    pub fn dbirth_file_path(&self) -> &str {
        &self.dbirth_file_path
    }
    pub fn ddata_file_path(&self) -> &str {
        &self.ddata_file_path
    }pub fn ddeath_file_path(&self) -> &str {
        &self.ddeath_file_path
    }

    pub fn mqtt_broker_hostname(&self) -> &str {
        &self.mqtt_broker_hostname
    }

    pub fn number_devices_to_simulate(&self) -> f64 {
        self.number_devices_to_simulate
    }


    // ... other getters ...

    // Setters
    pub fn set_mqtt_broker_hostname(&mut self, hostname: String) {
        self.mqtt_broker_hostname = hostname;
    }

    pub fn set_number_devices_to_simulate(&mut self, number: f64) {
        self.number_devices_to_simulate = number;
    }
pub fn set_dbirth_file_path(&mut self, dbirth_file_path: String) {
    self.dbirth_file_path = dbirth_file_path;
}
    pub fn set_ddata_file_path(&mut self, ddata_file_path: String) {
        self.ddata_file_path= ddata_file_path;
    }
    pub fn set_ddeath_file_path(&mut self, ddeath_file_path: String) {
        self.ddeath_file_path= ddeath_file_path;
    }
    // ... other setters ...
}
