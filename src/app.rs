use egui::RichText;
use rand::seq::SliceRandom;
use rand::thread_rng;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    tab: usize,
    last_tab: usize,
    input_text: String,
    output_text: String,
    cypher_key: [char; 26],
    cypher_key_string: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tab: 0,
            last_tab: 0,
            input_text: String::new(),
            output_text: String::new(),
            cypher_key: ('A'..='Z').collect::<Vec<_>>().try_into().unwrap(), // Default is just a vector with default alphabet order
            cypher_key_string: String::new(),
        }
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

    // Encryptin Function
    fn encrypt(&self, text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let is_upper = c.is_ascii_uppercase();
                    let idx = c.to_ascii_uppercase() as usize - 'A' as usize; // Map char to correct index
                    let sub = self.cypher_key[idx]; // Substitute using key
                    if is_upper {
                        sub
                    } else {
                        sub.to_ascii_lowercase()
                    }
                } else {
                    c // Non-alphabetic chars don't change
                }
            })
            .collect()
    }

    // Decryption Function
    fn decrypt(&self, text: &str) -> String {
        let mut reverse_key = ['A'; 26];
        for (i, &c) in self.cypher_key.iter().enumerate() {
            reverse_key[c as usize - 'A' as usize] = (b'A' + i as u8) as char; // Reverse mapping from cypher char to original letters
        }

        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let is_upper = c.is_ascii_uppercase();
                    let idx = c.to_ascii_uppercase() as usize - 'A' as usize; // Map char to correct index
                    let sub = reverse_key[idx]; // Substitute using key
                    if is_upper {
                        sub
                    } else {
                        sub.to_ascii_lowercase()
                    }
                } else {
                    c // Non-alphabetic chars don't change
                }
            })
            .collect()
    }

    // Randomizes Key
    fn randomize_key(&mut self) {
        let mut rng = thread_rng();
        let mut letters: Vec<char> = ('A'..='Z').collect();
        letters.shuffle(&mut rng); // Randomizes all the chars in the letters Vec
        self.cypher_key = letters.try_into().unwrap();
    }

    // Ensures that key is valid
    fn is_valid_key(&self) -> bool {
        let key = self.cypher_key_string.to_uppercase();
        key.len() == 26 && key.chars().all(|c| c.is_ascii_uppercase()) && {
            let mut seen = std::collections::HashSet::new();
            key.chars().all(|c| seen.insert(c))
        }
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
                    // This ensures that text boxes don't persist between tab switches
                    if self.tab != self.last_tab {
                        self.input_text.clear();
                        self.output_text.clear();
                        self.last_tab = self.tab;
                    }
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
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center),
                |ui| match self.tab {
                    0 => {
                        ui.label(RichText::new("About").heading().size(24.0).strong());
                        ui.add_space(10.0);
                        ui.label("Mono-alphabetic substitution cipher implementation in Rust using egui. Can generate random keys to use, and allows for encryption and decryption.");
                    }
                    1 => {
                        ui.label(RichText::new("Encrypt").heading().size(24.0).strong());
                        ui.add_space(10.0);

                        ui.label("Current substitution key (A-Z):");
                        ui.text_edit_singleline(&mut self.cypher_key_string);
                        ui.add_space(5.0);
                        if !self.is_valid_key() {
                            ui.colored_label(egui::Color32::RED, "Invalid: Key must be 26 unique A-Z letters");
                        }

                        if ui.button("Randomize Key").clicked() {
                            self.randomize_key();
                            self.cypher_key_string = self.cypher_key.iter().collect();
                        }

                        ui.add_space(10.0);
                        ui.label("Enter text to encrypt:");
                        ui.text_edit_multiline(&mut self.input_text);

                        if ui.button("Encrypt").clicked() {
                            self.output_text = self.encrypt(&self.input_text);
                        }

                        ui.add_space(10.0);
                        ui.label("Encrypted text:");
                        ui.text_edit_multiline(&mut self.output_text);

                        if ui.button("📋").clicked() {
                            ctx.output_mut(|o| o.copied_text = self.output_text.clone());
                        }
                    }
                    2 => {
                        ui.label(RichText::new("Decrypt").heading().size(24.0).strong());
                        ui.add_space(10.0);

                        ui.label("Enter text to decrypt:");
                        ui.text_edit_multiline(&mut self.input_text);

                        if ui.button("Decrypt").clicked() {
                            self.output_text = self.decrypt(&self.input_text);
                        }

                        ui.add_space(10.0);
                        ui.label("Decrypted text:");
                        ui.text_edit_multiline(&mut self.output_text);
                    }
                    _ => {}
                },
            );
        });
    }
}
