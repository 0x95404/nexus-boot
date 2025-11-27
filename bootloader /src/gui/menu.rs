// bootloader/src/gui/menu.rs
use eframe::egui::{self, TextureHandle, Image, Sense};
use alloc::vec::Vec;
use std::path::PathBuf;

pub struct IsoGrid {
    pub textures: Vec<(PathBuf, TextureHandle)>,
}

impl IsoGrid {
    pub fn new() -> Self { Self { textures: Vec::new() } }

    pub fn ui(&mut self, ui: &mut egui::Ui, selected: &mut Option<PathBuf>) {
        ui.heading("ISO Library – Drag & Drop or Click");

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                // Scan USB root for .iso / .wim / .img
                let isos = vec!["Gandalf_Redstone8.iso", "Win11_24H2.iso", "Medicap.iso", "Kali.iso"];
                for iso_name in isos {
                    let path = PathBuf::from(iso_name);
                    let thumbnail = ui.ctx().load_texture(
                        iso_name, 
                        egui::ColorImage::from_rgba_unmultiplied([256, 256], &include_bytes!("../../../assets/icons/iso.png")[..]),
                        egui::TextureOptions::linear(),
                    );

                    let image = Image::new(&thumbnail, [140.0, 140.0])
                        .sense(Sense::click_and_drag());

                    if ui.add(image).drag_started() {
                        ui.output().cursor_icon = egui::CursorIcon::Grabbing;
                    }

                    if ui.add(image).clicked() {
                        *selected = Some(path.clone());
                    }

                    ui.vertical(|ui| {
                        ui.label(path.file_name().unwrap().to_string_lossy().to_string());
                        ui.small("14.2 GB · Win11 PE");
                    });
                    ui.add_space(20.0);
                }
            });
        });
    }
}
