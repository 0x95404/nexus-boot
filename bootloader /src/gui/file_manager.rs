use eframe::egui;
use std::path::PathBuf;

static mut CURRENT_DIR: PathBuf = PathBuf::new();

pub fn show(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("File Manager - Copy/Paste/Delete")
        .open(open)
        .resizable(true)
        .default_size([800.0, 600.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    unsafe { if let Some(parent) = CURRENT_DIR.parent() { CURRENT_DIR = parent.to_path_buf(); } }
                }
                ui.label(format!("Path: {}", unsafe { CURRENT_DIR.display() }));
            });

            ui.separator();

            // Dummy entries - will be replaced with real FS in Phase 6
            for entry in ["Gandalf_Redstone8.iso", "Win11.iso", "Drivers/", "PXE_Server/"].iter() {
                ui.horizontal(|ui| {
                    if ui.button("Open").clicked() { /* open logic */ }
                    ui.label(*entry);
                    if ui.button("Copy").clicked() { /* clipboard */ }
                    if ui.button("Delete").clicked() { /* delete */ }
                });
            }
        });
}
