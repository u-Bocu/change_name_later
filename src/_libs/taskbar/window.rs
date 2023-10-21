#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub fn show() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 80.0)),
        resizable: false,
        ..Default::default()
    };

    // Our application state:
    let mut french = "".to_owned();
    let mut chinese = "".to_owned();

    eframe::run_simple_native("Pirate's Parrot", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("French: ");
                ui.text_edit_singleline(&mut french)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Chinese: ");
                ui.text_edit_singleline(&mut chinese)
                    .labelled_by(name_label.id);
            });

            if ui.button("Add word").clicked() {}
        });
    })
}
