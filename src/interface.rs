use crate::balance::Balancer;
use crate::parser::{ChemicalEquation, Compound};
use crate::stoichiometry::{Reactant, StoichCalculator};
use eframe::egui::{Context, Ui, Visuals};
use eframe::{egui, Frame};

#[derive(Debug, PartialEq)]
enum StoichMode {
    ProductUnknown,
    ReactantUnknown,
}

pub(crate) struct App {
    eq_input: String,
    eq_display: ChemicalEquation,
    selected_stoich_mode: StoichMode,
    stoich_input_strings: Vec<(String, bool)>,
    stoich_input_reactants: Vec<Reactant>,
    stoich_calculator: StoichCalculator,
}

impl App {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        App {
            eq_input: String::new(),
            eq_display: ChemicalEquation::empty(),
            selected_stoich_mode: StoichMode::ProductUnknown,
            stoich_input_strings: Vec::new(),
            stoich_input_reactants: Vec::new(),
            stoich_calculator: StoichCalculator::new(ChemicalEquation::empty(), Vec::new()),
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
            display_chem_eq(ui, self);
            egui::ComboBox::from_label("Stoichiometry Mode")
                .selected_text(format!("{:?}", self.selected_stoich_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.selected_stoich_mode,
                        StoichMode::ProductUnknown,
                        "Product Unknown",
                    );
                    /*ui.selectable_value(
                        &mut self.selected_stoich_mode,
                        StoichMode::ReactantUnknown,
                        "Reactant Unknown",
                    );*/
                });
            while self.stoich_input_strings.len() < self.eq_display.terms.len() {
                self.stoich_input_strings.push((String::new(), false));
            }
            while self.stoich_input_reactants.len() < self.eq_display.terms.len() {
                self.stoich_input_reactants.push(Reactant::None);
            }
            match self.selected_stoich_mode {
                StoichMode::ProductUnknown => {
                    for (i, cpd) in self.eq_display.terms[0..self.eq_display.rhs_ix]
                        .iter()
                        .enumerate()
                    {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.stoich_input_strings[i].0)
                                    .hint_text(format!("{}", cpd.raw()))
                                    .desired_width(60.0),
                            );
                            let input = &mut self.stoich_input_strings[i]
                                .0
                                .parse::<f32>()
                                .unwrap_or_default();
                            egui::ComboBox::new(i, "")
                                .selected_text(self.stoich_input_reactants[i].list_display())
                                .width(45.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.stoich_input_reactants[i],
                                        Reactant::Grams(*input),
                                        "g",
                                    );
                                    ui.selectable_value(
                                        &mut self.stoich_input_reactants[i],
                                        Reactant::Moles(*input),
                                        "mol",
                                    );
                                });
                            ui.checkbox(&mut self.stoich_input_strings[i].1, "sufficient?");
                            if self.stoich_input_strings[i].1 {
                                self.stoich_input_reactants[i] = Reactant::Excess;
                            } else {
                                self.stoich_input_reactants[i] =
                                    match &self.stoich_input_reactants[i] {
                                        Reactant::Grams(_) => Reactant::Grams(*input),
                                        Reactant::Moles(_) => Reactant::Moles(*input),
                                        react => react.clone(),
                                    }
                            }
                            self.stoich_calculator.eq = self.eq_display.clone();
                            self.stoich_calculator.inputs = self.stoich_input_reactants.clone();
                        });
                    }
                }
                StoichMode::ReactantUnknown => (),
            }
            if ui.button("Stoich Time!").clicked() {
                self.stoich_calculator.product_unknown();
            }
            for op in &self.stoich_calculator.outputs {
                ui.label(op.to_string());
            }
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

fn display_chem_eq(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        let mut iter = app.eq_display.terms[0..app.eq_display.rhs_ix]
            .iter()
            .peekable();
        while let Some(cpd) = iter.next() {
            display_cpd(ui, cpd);
            if let None = iter.peek() {
                ui.label("=");
                break;
            }
            ui.label("+");
        }
        let mut iter = app.eq_display.terms[app.eq_display.rhs_ix..app.eq_display.terms.len()]
            .iter()
            .peekable();
        while let Some(cpd) = iter.next() {
            display_cpd(ui, cpd);
            if let Some(_) = iter.peek() {
                ui.label("+");
            }
        }
    });
}
fn display_cpd(ui: &mut Ui, cpd: &Compound) {
    ui.vertical(|ui| {
        ui.label(cpd.to_string());
        ui.label(format!("{:.2}", cpd.molar_mass));
    });
}
