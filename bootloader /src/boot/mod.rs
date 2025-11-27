pub mod menu;
use menu::IsoGrid;

#[derive(Default)]
pub struct NexusApp {
    pub selected_iso: Option<std::path::PathBuf>,
    pub show_file_manager: bool,
    pub show_driver_injector: bool,
    pub iso_grid: IsoGrid,
}

// Inside update() function, replace the old drag zone with:
self.iso_grid.ui(ui, &mut self.selected_iso);
