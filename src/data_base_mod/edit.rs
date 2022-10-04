use crate::{
    eframe_tools::modal_machines::{act_on_tooth, modal_machine, ModalMachineGear},
    errors::client_errors::ErrorMessage,
    tools::{handle_load_result, load_file_as_string, string_to_json},
};
use std::{fs::OpenOptions, io::Write};

pub fn edit(
    text_file: &mut String,
    filenames: &Vec<String>,
    file_to_edit_modal: &mut String,
    error_message: &mut ErrorMessage,
    ui: &mut eframe::egui::Ui,
) {
    if ui.button("Beautify...!").clicked() {
        match string_to_json(text_file.to_string()) {
            Ok(json_string) => match serde_json::to_string_pretty(&json_string) {
                Ok(pretty_string) => {
                    *text_file = pretty_string;
                    *error_message = ErrorMessage::default();
                }
                Err(error) => {
                    *error_message = ErrorMessage::pure_error_message(Some(error.to_string()));
                }
            },
            Err(error) => {
                *error_message = ErrorMessage::pure_error_message(Some(error.to_string()));
            }
        }
    }
    if ui.button("Save...!?").clicked() {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("postgresql_json/{}", file_to_edit_modal))
        {
            Ok(mut file) => match file.write_all(text_file.as_bytes()) {
                Ok(_) => println!("Sucessfully wrote/saved text to file"),
                Err(error) => {
                    *error_message = ErrorMessage::pure_error_message(Some(error.to_string()))
                }
            },
            Err(error) => {
                *error_message = ErrorMessage::pure_error_message(Some(error.to_string()));
            }
        }
    }
    eframe::egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add(eframe::egui::TextEdit::multiline(text_file));
    });
    let tooth = modal_machine(
        file_to_edit_modal,
        ui,
        ModalMachineGear::Immutable(filenames),
        2,
    );
    act_on_tooth(tooth, |some_option| {
        let result = load_file_as_string(&format!("postgresql_json/{}", some_option));
        handle_load_result(result, text_file, error_message);
    });
}
