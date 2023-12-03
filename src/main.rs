extern crate nalgebra as na;

use std::sync::Arc;

mod balance;
mod interface;
mod matrix;
mod parser;
mod stoichiometry;

fn main() -> Result<(), eframe::Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.icon = Some(Arc::new(interface::load_icon()));
    eframe::run_native(
        "stoic",
        native_options,
        Box::new(|cc| Box::new(interface::App::new(cc))),
    )
}
