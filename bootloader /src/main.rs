#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use limine_uefi::LimineBootServices;
use uefi::prelude::*;
use eframe::egui;
use crate::gui::{NexusApp, apply_theme};

mod gui;
mod boot;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("PANIC: {}", info);
    loop {}
}

#[global_allocator]
static ALLOC: uefi::alloc::Allocator = uefi::alloc::Allocator;

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    panic!("Memory allocation failed");
}

#[entry]
fn efi_main(image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&system_table).unwrap();
    log::info!("NEXUS Boot v0.1 - The End of Ventoy");

    let bt = system_table.boot_services();

    eframe::start_efi(image_handle, bt, |ctx| {
        apply_theme(ctx, "cyberpunk");
        Box::new(NexusApp::default())
    })
}
