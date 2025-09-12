use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_title("Napkin - Disposable Notes"),
        ..Default::default()
    };

    eframe::run_native(
        "Napkin",
        options,
        Box::new(|_cc| Box::<NapkinApp>::default()),
    )
}

struct NapkinApp {
    text: String,
    theme: Theme,
    show_info: bool,
}

#[derive(PartialEq)]
enum Theme {
    Light,
    Dark,
    Sepia,
    Blue,
}

impl Default for NapkinApp {
    fn default() -> Self {
        Self {
            text: String::new(),
            theme: Theme::Light,
            show_info: false,
        }
    }
}

impl eframe::App for NapkinApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for Ctrl+D key press to clear note
        if ctx.input(|i| i.key_pressed(egui::Key::D) && i.modifiers.ctrl) {
            self.text.clear();
        }

        // Apply theme
        match self.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Sepia => {
                let mut sepia = egui::Visuals::light();
                sepia.window_fill = egui::Color32::from_rgb(255, 240, 210);
                sepia.panel_fill = egui::Color32::from_rgb(255, 245, 225);
                ctx.set_visuals(sepia);
            },
            //Whole ass blue sea color.
            Theme::Blue => {
                let mut blue = egui::Visuals::light();
                blue.window_fill = egui::Color32::from_rgb(8,76,143);
                blue.panel_fill = egui::Color32::from_rgb(8,143,76);
                ctx.set_visuals(blue);
            }
        }

        // Top menu bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // NEW button
                if ui.button("🆕 NEW").clicked() {
                    self.text.clear();
                }
                
                // THEMES dropdown
                ui.menu_button("🎨 THEMES", |ui| {
                    if ui.button("☀️ Light").clicked() {
                        self.theme = Theme::Light;
                        ui.close_menu();
                    }
                    if ui.button("🌙 Dark").clicked() {
                        self.theme = Theme::Dark;
                        ui.close_menu();
                    }
                    if ui.button("📜 Sepia").clicked() {
                        self.theme = Theme::Sepia;
                        ui.close_menu();
                    }
                    //The Sea Blue is self implemented, i hope i implemented it correctly.
                    if ui.button("🌊 Sea Blue").clicked(){
                        self.theme = Theme::Blue;
                        ui.close_menu();
                    }
                });
                
                // INFO button
                if ui.button("ℹ️ INFO").clicked() {
                    self.show_info = true;
                }
            });
        });

        // Info modal // UPDATE: 0.2 added license
        if self.show_info {
            egui::Window::new("About Napkin")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.heading("Napkin - Disposable Notes");
                    ui.separator();
                    ui.label("A temporary notepad for quick thoughts");
                    ui.label("Press Ctrl+D to clear the current note");
                    ui.label("Notes are not saved - they disappear when you close the app");
                    ui.label("Use the top menu to change themes or get info");
                    ui.label("Napkin is licensed under the MIT License.");
                    ui.label("Made in rust with love :3");
                    ui.separator();
                    if ui.button("OK").clicked() {
                        self.show_info = false;
                    }
                });
        }

        // Main editor area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Text editor with nice styling
            let text_edit = egui::TextEdit::multiline(&mut self.text)
                .desired_width(f32::INFINITY)
                .desired_rows(25)
                .font(egui::TextStyle::Monospace)
                .frame(true);
            
            ui.add(text_edit);
            
            // Update 0.2: Removed Status Bar(useless)
        });

        ctx.request_repaint();
    }
}