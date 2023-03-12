#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cawr_infrastructure::{desktop, logger};
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    logger::init_default_logger();
    Ok(desktop::run()?)
}
