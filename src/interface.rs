use crate::balance::Balancer;
use crate::parser::{parse, ChemicalEquation};
use eframe::egui::{Context, Key, Visuals};
use eframe::{egui, Frame};

pub(crate) struct App {
    eq_input: String,
    eq_display: String,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        App {
            eq_input: String::new(),
            eq_display: String::new(),
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
                self.eq_display = Balancer::balance_real_time(&self.eq_input);
            });
            ui.add_space(10.0);
            ui.label(&self.eq_display);
        });
    }
}

pub(crate) fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
