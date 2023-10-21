#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::mem::zeroed;

// Local module
use pirate_parrot_lib::*;

fn main() -> Result<(), eframe::Error> {
    let hwnd: HWND;
    #[cfg(target_family = "windows")]
    {
        hwnd = unsafe { taskbar::create() }
    }

    // Main loop
    loop {
        if unsafe { taskbar::EXIT == true } {
            break;
        }

        let mut msg: MSG = unsafe { zeroed() };
        #[cfg(target_family = "windows")]
        unsafe {
            PeekMessageA(&mut msg, hwnd, WM_RBUTTONDOWN, WM_RBUTTONDOWN, PM_REMOVE)
        };
    }

    Ok(())
}
