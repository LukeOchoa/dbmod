mod data_base_mod;
mod errors;

pub use data_base_mod::*;
pub use errors::client_errors::{ErrorMessage, WindowMessage};

mod tools {
    use crate::errors::client_errors::ErrorMessage;
    use postgres::{Client, NoTls};
    use std::collections::HashMap;

    pub fn load_file_as_string(relative_path: &str) -> Result<String, String> {
        match file_to_string(relative_path) {
            Ok(some_string) => return Ok(some_string),
            Err(error) => return Err(error.to_string()),
        }
    }
    pub fn handle_load_result(
        some_result: Result<String, String>,
        file_holder: &mut String,
        error_message: &mut ErrorMessage,
    ) {
        match some_result {
            Ok(some_string) => *file_holder = some_string,
            Err(error_string) => {
                *error_message = ErrorMessage::pure_error_message(Some(error_string))
            }
        }
    }

    pub fn db_connection() -> Result<Client, String> {
        let result = Client::connect(
            "host=localhost port=5432 dbname=breaker user=luke password=free144",
            NoTls,
        );
        match result {
            Ok(client) => return Ok(client),
            Err(error) => return Err(error.to_string()),
        }
    }
    pub fn serde_string_to_map(some_value: String) -> Result<HashMap<String, String>, String> {
        let maybe_map = serde_json::from_str(&some_value);
        match maybe_map {
            Ok(map) => return Ok(map),
            Err(error) => return Err(error.to_string()),
        }
    }
    // TODO
    pub fn serde_string_to_map_with_array(
        some_value: String,
    ) -> Result<HashMap<String, Vec<String>>, String> {
        let maybe_map_array = serde_json::from_str(&some_value);
        match maybe_map_array {
            Ok(map) => return Ok(map),
            Err(error) => return Err(error.to_string()),
        }
    }

    pub fn file_to_string(path: &str) -> Result<String, std::io::Error> {
        let result = std::fs::read_to_string(path);

        result
    }

    pub fn string_to_json(some_string: String) -> Result<serde_json::Value, serde_json::Error> {
        let res = serde_json::from_str(&some_string);

        res
    }

    pub fn postgresql_json_filenames() -> Result<Vec<String>, ErrorMessage> {
        match current_dir_string() {
            Ok(current_dir) => match file_names(current_dir, "/postgresql_json/".to_string()) {
                Ok(filenames) => return Ok(filenames),
                Err(error_string) => {
                    return Err(ErrorMessage::pure_error_message(Some(error_string)))
                }
            },
            Err(error_string) => return Err(ErrorMessage::pure_error_message(Some(error_string))),
        }
    }

    pub fn current_dir_string() -> Result<String, String> {
        match std::env::current_dir() {
            Ok(current_dir) => return option_str_to_string(current_dir.to_str()),
            Err(_) => {
                return Err("Could not obtain the current directory".to_string());
            }
        }
    }

    fn option_str_to_string(option_str: Option<&str>) -> Result<String, String> {
        match option_str {
            Some(a_str) => return Ok(a_str.to_string()),
            None => return Err("Could not convert current_dir(type: PathBuf to &str".to_string()),
        }
    }
    // returns a list of filenames from the specified folder of a specific directory
    pub fn file_names(directory: String, folder: String) -> Result<Vec<String>, String> {
        let path = format!("{}{}", directory, folder);
        let mut filenames = Vec::new();
        match std::fs::read_dir(path) {
            Ok(dir_iter) => {
                for maybe_entry in dir_iter {
                    match maybe_entry {
                        Ok(entry) => match option_str_to_string(entry.file_name().to_str()) {
                            Ok(some_string) => filenames.push(some_string),
                            Err(error_string) => return Err(error_string),
                        },
                        Err(error) => return Err(error.to_string()),
                    }
                }
                return Ok(filenames);
            }
            Err(error) => return Err(error.to_string()),
        }
    }
}

mod eframe_tools {
    use crate::errors::client_errors::{ErrorMessage, WindowMessage};
    pub fn handle_errors(
        an_error: &mut ErrorMessage,
        ctx: &eframe::egui::Context,
        ui: &mut eframe::egui::Ui,
    ) {
        an_error.is_window_open = an_error.display_error(ctx);
        an_error.impure_open_error_window_on_click(ui);
    }
    pub fn handle_window_message(
        a_message: &mut WindowMessage,
        ctx: &eframe::egui::Context,
        ui: &mut eframe::egui::Ui,
    ) {
        a_message.is_window_open = a_message.display_message(ctx);
        a_message.open_window_on_click(ui);
    }

    pub mod modal_machines {

        type Tooth = Option<String>;

        pub enum ModalMachineGear<'a> {
            Constant(&'static Vec<String>),
            Immutable(&'a Vec<String>),
            Mutable(&'a mut Vec<String>),
        }
        pub fn modal_machine(
            selected_modal: &mut String,
            ui: &mut eframe::egui::Ui,
            //const_page_options: &'static Vec<String>,
            gear: ModalMachineGear,
            ui_id: i32,
        ) -> Tooth {
            let mut some_tooth: Tooth = None;
            ui.push_id(ui_id, |ui| {
                eframe::egui::ComboBox::from_label("Choose a Modal...!")
                    .selected_text(selected_modal.clone())
                    .show_ui(ui, |ui| {
                        let mut wheel = |some_gear: &Vec<String>| {
                            for tooth in some_gear {
                                if ui
                                    .selectable_value(selected_modal, tooth.to_string(), tooth)
                                    .clicked()
                                {
                                    some_tooth = Some(tooth.to_string())
                                }
                            }
                        };
                        match gear {
                            ModalMachineGear::Constant(some_constant_gear) => {
                                wheel(some_constant_gear);
                            }
                            ModalMachineGear::Immutable(some_immutable_gear) => {
                                wheel(some_immutable_gear);
                            }
                            ModalMachineGear::Mutable(some_mutable_gear) => {
                                wheel(some_mutable_gear);
                            }
                        }
                    });
            });

            some_tooth
        }

        pub fn act_on_tooth(some_tooth: Tooth, mut action: impl FnMut(&str)) {
            if let Some(tooth) = some_tooth {
                action(&tooth);
            }
        }
    }
}
