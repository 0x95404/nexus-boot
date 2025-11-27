#![no_main]
#![no_std]
#![feature(abi_efiapi)]

mod gui;
mod boot;
mod pxe;

use core::panic::PanicInfo;
use limine_uefi::LimineBootServices;
use uefi::prelude::*;
use eframe::egui;
use gui::{NexusApp, Theme};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[entry]
fn efi_main(image: Handle, st: SystemTable<Boot>) -> Status {
    let bt = st.boot_services();
    eframe::start_efi(image, bt, |ctx| {
        gui::apply_cyberpunk_theme(ctx);
        Box::new(NexusApp::default())
    })
}
