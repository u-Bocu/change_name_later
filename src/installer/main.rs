#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::mem::zeroed;

// Local module
use pirate_parrot_lib::*;

fn main() -> Result<(), eframe::Error> {
    Ok(())
}
