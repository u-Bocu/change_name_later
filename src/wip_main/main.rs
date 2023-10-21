#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use eframe::egui;
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

        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(320.0, 240.0)),
            ..Default::default()
        };

        eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("My egui Application");
                ui.label(format!("Hello"));
            });
        })?;
    }

    Ok(())
}
