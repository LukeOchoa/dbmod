// std library imports
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// other
pub use crate::errors::client_errors::{ErrorMessage, WindowMessage};

use crate::{
    edit::edit,
    eframe_tools::{
        handle_errors, handle_window_message,
        modal_machines::{self, act_on_tooth},
    },
    rebuild::{rebuild, Rebuild},
    tools::{
        handle_load_result, load_file_as_string, postgresql_json_filenames, serde_string_to_map,
        serde_string_to_map_with_array,
    },
};

// emilk imports
use eframe::egui;
use postgres::error;

// Globals
//static START: Once = Once::new();

pub struct DataBaseMod {
    pub error_message: Arc<Mutex<ErrorMessage>>,
    tables_file: String,
    rows_file: String,
    some_files: Vec<String>,
    chosen_operation_modal: String,
    file_to_edit_modal: String,
    some_rebuild: Rebuild,
}
//fn load_file_as_string(relative_path: &str) -> Result<String, String> {
//    match file_to_string(relative_path) {
//        Ok(some_string) => return Ok(some_string),
//        Err(error) => return Err(error.to_string()),
//    }
//}
//fn handle_load_result(
//    some_result: Result<String, String>,
//    file_holder: &mut String,
//    error_message: &mut ErrorMessage,
//) {
//    match some_result {
//        Ok(some_string) => *file_holder = some_string,
//        Err(error_string) => *error_message = ErrorMessage::pure_error_message(Some(error_string)),
//    }
//}

impl Default for DataBaseMod {
    fn default() -> Self {
        let error_messagex = Arc::new(Mutex::new(ErrorMessage::default()));

        let mut tables_filex = String::default();
        let result = load_file_as_string("postgresql_json/breaker_db_tables.json");
        handle_load_result(
            result,
            &mut tables_filex,
            &mut error_messagex.lock().unwrap(),
        );

        let mut rows_filex = String::default();
        let result = load_file_as_string("postgresql_json/breaker_db_table_values.json");
        handle_load_result(result, &mut rows_filex, &mut error_messagex.lock().unwrap());

        Self {
            error_message: error_messagex,
            tables_file: tables_filex,
            rows_file: rows_filex,
            some_files: vec!["No files up yet".to_string()],
            chosen_operation_modal: String::default(),
            file_to_edit_modal: String::default(),
            some_rebuild: Rebuild::default(),
        }
    }
}

impl eframe::App for DataBaseMod {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            handle_errors(&mut self.error_message.lock().unwrap(), ctx, ui);
            let tooth = modal_machines::modal_machine(
                &mut self.chosen_operation_modal,
                ui,
                modal_machines::ModalMachineGear::Immutable(&vec![
                    "Edit".to_string(),
                    "Rebuild".to_string(),
                    "Delete".to_string(),
                ]),
                1,
            );
            act_on_tooth(tooth, |some_option| {
                // For Edit
                match postgresql_json_filenames() {
                    Ok(filenames) => {
                        self.some_files = filenames;
                        self.file_to_edit_modal = String::default();
                    }
                    Err(some_error_message) => {
                        *self.error_message.lock().unwrap() = some_error_message;
                    }
                }
                // For Rebuild
                if some_option == "Rebuild" {
                    // Update state held files...
                    let result = load_file_as_string("postgresql_json/breaker_db_tables.json");
                    handle_load_result(
                        result,
                        &mut self.tables_file,
                        &mut self.error_message.lock().unwrap(),
                    );
                    let result =
                        load_file_as_string("postgresql_json/breaker_db_table_values.json");
                    handle_load_result(
                        result,
                        &mut self.rows_file,
                        &mut self.error_message.lock().unwrap(),
                    );
                    // Get SQL tables
                    match serde_string_to_map(self.tables_file.clone()) {
                        Ok(tables_file_as_map) => {
                            self.some_rebuild.sql_tables = tables_file_as_map;
                        }
                        Err(error_string) => {
                            *self.error_message.lock().unwrap() =
                                ErrorMessage::pure_error_message(Some(error_string))
                        }
                    }
                    // Get SQL rows
                    match serde_string_to_map_with_array(self.rows_file.clone()) {
                        Ok(rows_file_as_map) => {
                            self.some_rebuild.sql_rows = rows_file_as_map;
                        }
                        Err(error_string) => {
                            *self.error_message.lock().unwrap() =
                                ErrorMessage::pure_error_message(Some(error_string))
                        }
                    }
                    // Build a vector to be used as a gear for the modal machine
                    self.some_rebuild.gear = Rebuild::build_mmg(&self.some_rebuild);
                }
            });
            match self.chosen_operation_modal.as_str() {
                "Edit" => {
                    edit(
                        &mut self.tables_file,
                        &self.some_files,
                        &mut self.file_to_edit_modal,
                        &mut self.error_message.lock().unwrap(),
                        ui,
                    );
                }
                "Rebuild" => {
                    handle_window_message(&mut self.some_rebuild.window_message, ctx, ui);
                    rebuild(&mut self.some_rebuild, ui, &mut self.error_message);
                }
                "Delete" => {}
                _ => {
                    ui.label("Please choose an operation to perform");
                }
            }
        });
    }
}
