/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tab: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { tab: 0 }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.add_space(8.0);

                    if ui.button("About").clicked() {
                        self.tab = 0;
                    }
                    ui.add_space(8.0);

                    if ui.button("Encrypt").clicked() {
                        self.tab = 1;
                    }
                    ui.add_space(8.0);

                    if ui.button("Decrypt").clicked() {
                        self.tab = 2;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tab == 0 {
                ui.heading("About");
            } else if self.tab == 1 {
                ui.heading("Encrypt");
            } else if self.tab == 2 {
                ui.heading("Decrypt");
            }
        });
    }
}
