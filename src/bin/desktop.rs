#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cawr_infrastructure::{desktop, logger};

pub fn main() {
    logger::init_default_logger();
    desktop::run();
}
