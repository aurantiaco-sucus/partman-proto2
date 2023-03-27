use log::info;
use uefi::prelude::*;
use uefi::Result;
use uefi::table::boot::ScopedProtocol;

static mut EFI_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

#[inline]
pub fn initialize(system_table: SystemTable<Boot>) {
    unsafe { EFI_SYSTEM_TABLE = Some(system_table); }
    boot_services().set_watchdog_timer(0, 0, None).unwrap();
}

#[inline]
pub fn boot_services() -> &'static BootServices {
    unsafe { EFI_SYSTEM_TABLE.as_ref().unwrap_unchecked().boot_services() }
}

#[inline]
pub fn handle_of<T: uefi::proto::Protocol>() -> Result<Handle> {
    boot_services().get_handle_for_protocol::<T>()
}

#[inline]
pub fn proto_exclusive<T: uefi::proto::Protocol>() -> Result<ScopedProtocol<'static, T>> {
    boot_services().open_protocol_exclusive::<T>(handle_of::<T>()?)
}