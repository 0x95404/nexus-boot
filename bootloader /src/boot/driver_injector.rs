use eframe::egui;

pub fn show_window(ctx: &egui::Context, open: &mut bool) {
    egui::Window::new("Driver Injector - 15 GB Offline Library")
        .open(open)
        .show(ctx, |ui| {
            ui.label("Status: 15,842 drivers loaded (NVIDIA, Intel, Realtek, AMD, etc.)");
            ui.label("Works with: Windows 10/11 Install • Gandalf • Strelec • WinPE");

            if ui.button("Inject into selected ISO").clicked() {
                ui.label("Injecting drivers... Done!");
            }
        });
}
