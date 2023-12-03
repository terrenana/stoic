extern crate nalgebra as na;

mod balance;
mod interface;
mod matrix;
mod parser;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "stoic",
        native_options,
        Box::new(|cc| Box::new(interface::App::new(cc))),
    )
}
