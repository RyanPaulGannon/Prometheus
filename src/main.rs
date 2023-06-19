#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Prometheus",
        options,
        Box::new(|_cc| Box::<Prometheus>::default()),
    )
}

struct Prometheus {
    input_buffer: String,
    command_output: String,
}

impl Default for Prometheus {
    fn default() -> Self {
        Self {
            input_buffer: String::new(),
            command_output: String::new()
        }
    }
}

impl eframe::App for Prometheus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label(">");
                ui.text_edit_singleline(&mut self.input_buffer)
                    .labelled_by(name_label.id);

                if ui.button("Execute").clicked() {
                    if let Ok(output) = Command::new("sh")
                        .arg("-c")
                        .arg(&self.input_buffer)
                        .output()
                    {
                        if output.status.success() {
                            if let Ok(stdout) = String::from_utf8(output.stdout) {
                                self.command_output = stdout;
                            }
                        } else {
                            if let Ok(stderr) = String::from_utf8(output.stderr) {
                                self.command_output = stderr;
                            }
                        }
                    } else {
                        self.command_output = String::from("Failed to execute command");
                    }
                }

                ui.label(&self.command_output);
            });
        });
    }
}

