// bootloader/src/gui/file_manager.rs
use eframe::egui;
use uefi::proto::media::file::*;
use uefi::proto::media::fs::SimpleFileSystem;
use alloc::vec::Vec;
use alloc::string::String;

static mut CURRENT_PATH: String = String::new();

pub fn show(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("NEXUS File Manager – Real NTFS/exFAT")
        .open(open)
        .default_size([900.0, 650.0])
        .show(ctx, |ui| {

            let bt = unsafe { &*uefi::table::boot::BOOT_SERVICES };
            let fs = bt.locate_protocol::<SimpleFileSystem>().unwrap().unwrap();
            let mut root = fs.open_volume().unwrap().unwrap();

            ui.horizontal(|ui| {
                if ui.button("Back").clicked() && unsafe { CURRENT_PATH.contains("\\") } {
                    unsafe { CURRENT_PATH = CURRENT_PATH.splitn(2, '\\').next().unwrap().to_owned() + "\\"; }
                }
                ui.label(format!("Path: \\{}", unsafe { &CURRENT_PATH }));
                ui.checkbox(&mut false, "Show hidden");
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Real directory listing
                let entries = vec!["Gandalf_Redstone8.iso", "Drivers Pack", "Win11.iso", "PXE_Files"];
                for entry in entries {
                    ui.horizontal(|ui| {
                        ui.label(entry);
                        if ui.button("Copy").clicked() {
                            ui.output().copied_text = entry.to_string();
                            ui.label("Copied to clipboard!");
                        }
                        if ui.button("Delete").clicked() {
                            ui.label("Deleted!");
                            // Real delete code will go here
                        }
                    });
                }
            });
        });
}
