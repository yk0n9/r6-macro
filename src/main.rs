#![windows_subsystem = "windows"]

use std::fs::read_to_string;
use std::thread;

use eframe::egui::{Vec2, ViewportBuilder};
use eframe::NativeOptions;

use crate::ui::{LEAD_TIME, mouse};
use crate::ui::r#macro::Macro;

mod ui;
mod weapon;

fn main() {
    let viewport = ViewportBuilder {
        resizable: Some(false),
        inner_size: Some(Vec2::new(400.0, 500.0)),
        maximize_button: Some(false),
        ..Default::default()
    };
    let op = NativeOptions {
        viewport,
        ..Default::default()
    };
    thread::spawn(move || unsafe {
        mouse();
    });
    eframe::run_native("R6 Macro", op, Box::new(|cc| unsafe {
        let mut r6 = Macro::new(cc);
        if let Ok(file) = read_to_string("r6-config.txt") {
            r6.config = ron::from_str(&file).unwrap_or_default();
            LEAD_TIME = r6.config.lead_time;
        }
        Box::new(r6)
    })).ok();
}
