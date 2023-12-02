extern crate nalgebra as na;

mod parser;
mod balance;
mod g_elim;

use std::fmt::{Display};
use std::io::{stdin,Write};
use crate::balance::balance;
use crate::parser::Equation;
use eframe::{egui, Frame};
use eframe::egui::Context;

struct App {
    balance_input: String,
    balance_output: String,
}

fn main() -> Result<(), eframe::Error>{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("stoic", native_options, Box::new(|cc| Box::new(App::new(cc))))
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        App {
            balance_input: String::new(),
            balance_output: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("stoic");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.balance_input);
                if ui.button("Balance!").clicked() {
                    let eq = Equation::from(self.balance_input.clone());
                    self.balance_output = balance(eq).iter().map(|f| f.to_string() +", ").collect::<String>()
                }
            });
            ui.label(&self.balance_output);

        });
    }
}