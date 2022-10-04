use eframe::egui;

#[derive(Default)]
pub struct WindowMessage {
    pub message: Option<String>,
    pub is_window_open: bool,
    pub try_to_open_window: bool,
}
impl WindowMessage {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
    pub fn window_message(message: Option<String>) -> WindowMessage {
        WindowMessage {
            message,
            is_window_open: true,
            try_to_open_window: false,
        }
    }
    pub fn open_window_on_click(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new(5).show(ui, |ui| {
            if ui.button("Open window message").clicked() {
                self.try_to_open_window = true;
            }
            if self.try_to_open_window {
                if let None = self.message {
                    ui.label("There are no errors...!");
                } else {
                    self.is_window_open = true;
                    self.try_to_open_window = false;
                }
            }
            ui.end_row();
        });
    }
    pub fn display_message(&self, ctx: &egui::Context) -> bool {
        let mut is_window_shut: bool = self.is_window_open;
        if let Some(message) = self.message.clone() {
            egui::Window::new("Window...!")
                .open(&mut is_window_shut)
                .show(ctx, |ui| {
                    ui.label(message);
                });
        }
        is_window_shut
    }
}

#[derive(Default)]
pub struct ErrorMessage {
    pub error: Option<String>,
    pub is_window_open: bool,
    pub try_to_open_window: bool,
}
impl ErrorMessage {
    pub fn default() -> Self {
        let instance: Self = Default::default();
        instance
    }
    pub fn impure_set_error_message(
        &mut self,
        error_message: Option<String>,
        is_window_open: bool,
    ) {
        self.error = error_message;
        self.is_window_open = is_window_open;
    }
    pub fn pure_error_message(error_message: Option<String>) -> ErrorMessage {
        ErrorMessage {
            error: error_message,
            is_window_open: true,
            try_to_open_window: false,
        }
    }
    pub fn impure_open_error_window_on_click(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new(3).show(ui, |ui| {
            if ui.button("Open error window").clicked() {
                self.try_to_open_window = true;
            }
            if self.try_to_open_window {
                if let None = self.error {
                    ui.label("There are no errors...!");
                } else {
                    self.is_window_open = true;
                    self.try_to_open_window = false;
                }
            }
            ui.end_row();
        });
    }
    pub fn display_error(&self, ctx: &egui::Context) -> bool {
        let mut is_window_shut: bool = self.is_window_open;
        if let Some(error_message) = self.error.clone() {
            egui::Window::new("ERROR...!")
                .open(&mut is_window_shut)
                .show(ctx, |ui| {
                    ui.label(error_message);
                });
        }
        is_window_shut
    }
    //pub fn error_job(error_reason: &str, error_string: String) -> ErrorMessage{}
}
