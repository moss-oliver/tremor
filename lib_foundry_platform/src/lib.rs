extern crate lib_foundry_common;
use lib_foundry_common::*;

#[cfg(windows)]
extern crate lib_foundry_win32;
use lib_foundry_win32::*;

#[cfg(windows)]
pub fn get_platform() -> PlatformWin32 {
    PlatformWin32::new()
}

#[cfg(windows)]
pub fn get_platform_event(window: &Win32Window) -> Option<PlatformEvent> {
    return get_win32_event(window);
}

#[cfg(windows)]
pub fn draw_bmp(window: &Win32Window, width:u32, height:u32, bmp: *const u8) {
    win32_draw_bmp(window, width, height, bmp);
}

#[cfg(unix)]
pub fn get_platform() -> Platform {
    platform::Unix
}

#[cfg(unix)]
pub fn get_platform_event() -> Option<PlatformEvent> {
    return Option::Some(PlatformEvent::WindowClose);
}