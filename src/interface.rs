use crate::balance::Balancer;
use crate::parser::ChemicalEquation;
use eframe::egui::{Context, Key, Visuals};
use eframe::{egui, Frame};

pub(crate) struct App {
    eq_input: String,
    balanced_eq: Option<ChemicalEquation>,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        App {
            eq_input: String::new(),
            balanced_eq: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("stoic");
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.eq_input).hint_text("Equation"));
                if ui.button("Balance!").clicked() || ui.ctx().input(|i| i.key_pressed(Key::Enter))
                {
                    self.balanced_eq = Some(Balancer::balance(&self.eq_input));
                }
            });
            ui.add_space(10.0);
            if let Some(eq) = &self.balanced_eq {
                ui.label(eq.to_string());
            }
        });
    }
}
