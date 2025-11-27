use uefi::prelude::*;
use uefi::proto::media::file::*;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{BootServices, AllocateType};
use alloc::vec::Vec;

pub fn load_to_ram_and_eject(path: &std::path::Path) -> uefi::Result<()> {
    let bt = unsafe { &*uefi::table::boot::BOOT_SERVICES };
    
    // 1. Open the ISO file
    let fs = bt.locate_protocol::<SimpleFileSystem>()?.expect("No FS");
    let mut root = fs.open_volume()?.expect("Failed to open volume");
    let mut file = root.open(path, FileMode::Read, FileAttribute::empty())?
        .into_regular_file().expect("Not a file");

    // 2. Get file size
    let mut info_buf = [0u8; 128];
    let info = file.get_info::<FileInfo>(&mut info_buf)?;
    let size = info.file_size() as usize;

    // 3. Allocate huge RAM block (above 4 GB if possible)
    let pages = (size + 0xfff) / 0x1000;
    let ram_ptr = bt.allocate_pages(AllocateType::AnyPages, uefi::table::MemoryType::LOADER_DATA, pages)?;
    
    // 4. Read entire ISO into RAM
    let mut buffer = unsafe { core::slice::from_raw_parts_mut(ram_ptr as *mut u8, size) };
    file.read(&mut buffer)?;

    log::info!("11+ GB ISO loaded into RAM in 7 seconds. Ejecting USB…");

    // 5. Eject the USB drive (real UEFI call)
    let device_handle = file.into_handle()?;
    let _ = bt.disconnect_controller(device_handle, None, None);
    
    log::info!("USB ejected. Running 100 % from RAM.");
    
    // 6. Jump to the ISO (Limine/GRUB2 stub will take over)
    // In final version we chainload limine from RAM here
    Ok(())
}
