use dbmod::data_base_mod_app::DataBaseMod;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "dbmod.io",
        options,
        Box::new(|_cc| Box::new(DataBaseMod::default())),
    );
}
