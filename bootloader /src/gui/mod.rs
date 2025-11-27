pub mod theme;
pub mod menu;
pub mod file_manager;

use eframe::egui;
use crate::boot::iso_loader;
use crate::pxe::server;

#[derive(Default)]
pub struct NexusApp {
    pub selected_iso: Option<String>,
    pub show_file_manager: bool,
    pub pxe_running: bool,
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("NEXUS Boot v0.1");
                ui.label("Drag ISO here • Mouse • Copy/Paste • 15 GB Drivers");

                ui.add_space(20.0);

                // Drag & drop ISO area
                if ui.add(egui::Button::new("Drop ISO here")).hovered() {
                    if let Some(file) = ctx.input().raw.dropped_files.first() {
                        self.selected_iso = Some(file.path.as_ref().unwrap().display().to_string());
                    }
                }

                if let Some(ref iso) = self.selected_iso {
                    ui.label(format!("Selected: {}", iso.file_name().unwrap().to_string_lossy()));
                    if ui.button("Load to RAM + Eject USB").clicked() {
                        iso_loader::load_to_ram_and_eject(iso).unwrap_or_else(|e| log::error!("{e}"));
                    }
                }

                ui.add_space(15.0);

                if ui.button("Open File Manager (Copy/Paste)").clicked() {
                    self.show_file_manager = true;
                }

                if ui.button(if self.pxe_running { "PXE Server Running" } else { "Start PXE Server (Boot LAN)" }).clicked() {
                    self.pxe_running = !self.pxe_running;
                    if self.pxe_running { server::start_pxe_server(); }
                }

                ui.add_space(30.0);
                ui.hyperlink_to("github.com/yourname/nexus-boot", "https://github.com/yourname/nexus-boot");
            });
        });

        if self.show_file_manager {
            file_manager::show_file_manager(ctx, &mut self.show_file_manager);
        }
    }
}

pub fn apply_cyberpunk_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.extreme_background_color = egui::Color32::from_rgb(13, 17, 23);
    visuals.panel_fill = egui::Color32::from_rgb(13, 17, 23);
    ctx.set_visuals(visuals);
}
