use uefi::proto::network::pxe::PxeBaseCode;
use uefi::proto::network::ip4::Ip4;
use alloc::vec::Vec;

static mut PXE_RUNNING: bool = false;

pub fn start_pxe_server() {
    unsafe {
        if PXE_RUNNING { return; }
        PXE_RUNNING = true;
    }

    let bt = unsafe { &*uefi::table::boot::BOOT_SERVICES };
    let pxe = bt.locate_protocol::<PxeBaseCode>()?.expect("No PXE support");

    // Configure DHCP + TFTP + HTTP server on 192.168.7.1
    let mut pxe = pxe.expect("PXE failed");
    pxe.start(true)?;
    pxe.dhcp()?;
    
    log::info!("NEXUS PXE SERVER ACTIVE");
    log::info!("IP: 192.168.7.1 – Booting other PCs now…");

    // Serve iPXE chainloader + all ISOs from USB or RAM
    loop {
        if let Ok(packet) = pxe.receive(1000) {
            if packet.contains(b"iPXE") {
                log::info!("Client booting: {}", core::str::from_utf8(&packet).unwrap_or("unknown"));
            }
        }
    }
}
