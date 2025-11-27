use eframe::egui::{self, Color32, Visuals};

pub fn apply_theme(ctx: &egui::Context, name: &str) {
    let mut visuals = Visuals::dark();

    match name {
        "cyberpunk" => {
            visuals.extreme_background_color = Color32::from_rgb(13, 17, 23);
            visuals.panel_fill = Color32::from_rgb(20, 25, 35);
            visuals.widgets.hovered.bg_fill = Color32::from_rgb(0, 255, 150);
            visuals.widgets.active.bg_fill = Color32::from_rgb(0, 200, 100);
        }
        "light" => ctx.set_visuals(Visuals::light()),
        "dark" => ctx.set_visuals(Visuals::dark()),
        _ => {}
    }

    ctx.set_visuals(visuals);
}
