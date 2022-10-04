use crate::{
    eframe_tools::modal_machines::{modal_machine, ModalMachineGear},
    tools::{current_dir_string, db_connection, file_names},
    ErrorMessage, WindowMessage,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct Rebuild {
    pub modal: String,
    pub add_rows: bool,
    pub sql_tables: HashMap<String, String>,
    pub sql_rows: HashMap<String, Vec<String>>,
    pub window_message: WindowMessage,
    pub gear: Vec<String>,
}

impl Rebuild {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
    // get_sql_table_string() returns a rust String of SQL
    // It uses the current modal/table the user has chosen as the SQL rust String it will return
    pub fn get_sql_table_string(&self) -> &String {
        self.sql_tables.get(&self.modal).unwrap()
    }
    pub fn get_sql_rows_string(&self) -> &Vec<String> {
        self.sql_rows.get(&self.modal).unwrap()
    }
    pub fn build_mmg(some_rebuild: &Rebuild) -> Vec<String> {
        let mut some_vec = Vec::new();
        for (key, _) in &some_rebuild.sql_tables {
            some_vec.push(key.to_string());
        }

        some_vec
    }
}

// Big Boi Function
pub fn rebuild(
    some_rebuild: &mut Rebuild,
    ui: &mut eframe::egui::Ui,
    error_message: &mut Arc<Mutex<ErrorMessage>>,
) {
    let gear = ModalMachineGear::Immutable(&some_rebuild.gear);

    ui.label("Which table would you like to rebuild...?");
    modal_machine(&mut some_rebuild.modal, ui, gear, 2);

    if ui.button(format!("{}", some_rebuild.add_rows)).clicked() {
        some_rebuild.add_rows = !some_rebuild.add_rows;
    }

    if some_rebuild.modal != String::default() {
        eframe::egui::Grid::new(4).show(ui, |ui| {
            ui.label(format!(
                "Are you sure you want to rebuild: |{}|",
                some_rebuild.modal
            ));
            if ui.button("Continue...?").clicked() {
                if let Err(error_string) = execute_rebuild(some_rebuild) {
                    *error_message.lock().unwrap() =
                        ErrorMessage::pure_error_message(Some(error_string));
                }
            }
            ui.end_row();
        });
    }
}

// Medium Boi Function
fn execute_rebuild(a_rebuild: &mut Rebuild) -> Result<(), String> {
    // Create DB Connection
    let mut client = db_connection()?;
    // Check if the requested rebuild table actually exists
    // if so delete it
    if check_if_table_exists(&mut client, &a_rebuild.modal)? {
        drop_table(&mut client, &a_rebuild.modal)?
    }
    // get sql string and rebuild the table
    let sql_string = a_rebuild.get_sql_table_string();
    let some_message = rebuild_table(&mut client, sql_string)?;
    // insert sql rows if the user requests
    let some_other_message = if a_rebuild.add_rows {
        let sql_string = a_rebuild.get_sql_rows_string();
        insert_rows_into_table(&mut client, sql_string)?
    } else {
        "".to_string()
    };

    // Handle any success messages
    a_rebuild.window_message =
        WindowMessage::window_message(Some(format!("{}\n{}", some_message, some_other_message)));

    // close connection to database
    if let Err(error) = client.close() {
        return Err(error.to_string());
    }
    Ok(())
}
fn rebuild_table(client: &mut postgres::Client, sql_string: &String) -> Result<String, String> {
    println!("\ndoes this work: \n{}", sql_string);
    match client.batch_execute(sql_string) {
        Ok(_) => return Ok("Table Successfully rebuilt".to_string()),
        Err(error) => return Err(error.to_string()),
    }
}
fn insert_rows_into_table(
    client: &mut postgres::Client,
    sql_vec_string: &Vec<String>,
) -> Result<String, String> {
    for sql_string in sql_vec_string {
        match client.batch_execute(&sql_string) {
            Ok(_) => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    return Ok("Rows Successfully added".to_string());
}
fn drop_table(client: &mut postgres::Client, table: &String) -> Result<(), String> {
    let drop_some_table = format!("DROP TABLE {};", table);
    match client.batch_execute(&drop_some_table) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error.to_string()),
    }
}
fn check_if_table_exists(client: &mut postgres::Client, table: &String) -> Result<bool, String> {
    let if_table_exists = format!(
        "SELECT EXISTS ( SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = '{}');",
        table
    );
    match client.query(&if_table_exists, &[]) {
        Ok(res) => {
            let some_bool: bool = res[0].get(0);
            return Ok(some_bool);
        }
        Err(error) => return Err(error.to_string()),
    }
}

// fn _display_filenames(ui: &mut eframe::egui::Ui, error_message: &mut ErrorMessage) {
//     match current_dir_string() {
//         Ok(current_dir) => match file_names(current_dir, "/postgresql_json/".to_string()) {
//             Ok(filenames) => {
//                 for filename in filenames {
//                     ui.label(filename);
//                 }
//             }
//             Err(error_string) => {
//                 *error_message = ErrorMessage::pure_error_message(Some(error_string))
//             }
//         },
//         Err(error_string) => *error_message = ErrorMessage::pure_error_message(Some(error_string)),
//     }
// }
