pub mod theme;
pub mod menu;
pub mod file_manager;

pub use theme::apply_theme;
use eframe::egui;
use crate::boot::{iso_loader, driver_injector};

#[derive(Default)]
pub struct NexusApp {
    pub selected_iso: Option<std::path::PathBuf>,
    pub show_file_manager: bool,
    pub show_driver_injector: bool,
    pub pxe_running: bool,
}

impl eframe::App for NexusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("NEXUS BOOT");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("File Manager").clicked() { self.show_file_manager = true; }
                    if ui.button("Driver Injector").clicked() { self.show_driver_injector = true; }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading("Drag & Drop Any ISO Below");
                ui.add_space(20.0);

                // Drag & Drop Target
                let drop_zone = egui::Frame::dark_body().inner_margin(40.0);
                drop_zone.show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Drop ISO / IMG / WIM here");
                        if let Some(picked_path) = &ctx.input().raw.dropped_files.first().and_then(|f| f.path.clone()) {
                            if picked_path.extension().map_or(false, |e| e == "iso" || e == "wim" || e == "img") {
                                self.selected_iso = Some(picked_path.clone());
                            }
                        }
                    });
                });

                if let Some(ref path) = self.selected_iso {
                    ui.add_space(15.0);
                    ui.label(format!("Ready: {}", path.file_name().unwrap().to_string_lossy()));
                    ui.horizontal(|ui| {
                        if ui.button("Load to RAM + Eject USB").clicked() {
                            let _ = iso_loader::load_to_ram_and_eject(path);
                        }
                        if ui.button("Boot Normally").clicked() {
                            let _ = iso_loader::boot_iso(path);
                        }
                    });
                }

                ui.add_space(30.0);
                ui.hyperlink_to("GitHub: NEXUS Boot", "https://github.com/yourname/nexus-boot");
            });
        });

        // Windows
        if self.show_file_manager {
            file_manager::show(ctx, &mut self.show_file_manager);
        }
        if self.show_driver_injector {
            driver_injector::show_window(ctx, &mut self.show_driver_injector);
        }
    }
}
