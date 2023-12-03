use crate::parser::{ChemicalEquation, Compound};
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Reactant {
    Grams(f32),
    Moles(f32),
    None,
    Excess,
}

impl Reactant {
    fn to_f32_moles(self, cpd: &Compound) -> f32 {
        match self {
            Reactant::Grams(i) => i / cpd.molar_mass,
            Reactant::Moles(i) => i,
            Reactant::Excess => f32::MAX,
            Reactant::None => 0.0,
        }
    }
    pub(crate) fn list_display(&self) -> String {
        match self {
            Reactant::Grams(_) => "g",
            Reactant::Moles(_) => "mol",
            _ => "",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StoichCalculator {
    pub(crate) eq: ChemicalEquation,
    pub(crate) inputs: Vec<Reactant>,
    pub(crate) outputs: Vec<f32>,
}

impl StoichCalculator {
    pub(crate) fn new(eq: ChemicalEquation, inputs: Vec<Reactant>) -> Self {
        StoichCalculator {
            eq,
            inputs,
            outputs: Vec::new(),
        }
    }
    pub(crate) fn product_unknown(&mut self) -> Self {
        let mut outputs = vec![0.0; self.eq.terms.len()];
        assert_eq!(self.eq.terms.len(), self.inputs.len());
        let mut limiting_unit_amt = f32::MAX;
        for (i, cpd) in self.eq.terms[0..self.eq.rhs_ix].iter().enumerate() {
            let product_produced =
                self.inputs[i].clone().to_f32_moles(&cpd) / cpd.coefficient as f32;
            if product_produced < limiting_unit_amt {
                limiting_unit_amt = product_produced;
            }
        }
        for (i, cpd) in self.eq.terms.iter().enumerate() {
            outputs[i] = match self.inputs[i] {
                Reactant::Grams(grams) => {
                    (grams / cpd.molar_mass) - limiting_unit_amt * cpd.coefficient as f32
                }
                Reactant::Moles(moles) => moles - limiting_unit_amt * cpd.coefficient as f32,
                Reactant::Excess => limiting_unit_amt * cpd.coefficient as f32 * -1.0,
                Reactant::None => limiting_unit_amt * cpd.coefficient as f32,
            }
        }
        self.outputs = outputs;
        self.clone()
    }
}

pub(crate) fn str_to_molar_mass(element: &str) -> f32 {
    match element {
        "H" => 1.00797,
        "He" => 4.00260,
        "Li" => 6.941,
        "Be" => 9.01218,
        "B" => 10.81,
        "C" => 12.011,
        "N" => 14.0067,
        "O" => 15.9994,
        "F" => 18.998403,
        "Ne" => 20.179,
        "Na" => 22.98977,
        "Mg" => 24.305,
        "Al" => 26.98154,
        "Si" => 28.0855,
        "P" => 30.97376,
        "S" => 32.06,
        "Cl" => 35.453,
        "K" => 39.0983,
        "Ar" => 39.948,
        "Ca" => 40.08,
        "Sc" => 44.9559,
        "Ti" => 47.90,
        "V" => 50.9415,
        "Cr" => 51.996,
        "Mn" => 54.9380,
        "Fe" => 55.847,
        "Ni" => 58.70,
        "Co" => 58.9332,
        "Cu" => 63.546,
        "Zn" => 65.38,
        "Ga" => 69.72,
        "Ge" => 72.59,
        "As" => 74.9216,
        "Se" => 78.96,
        "Br" => 79.904,
        "Kr" => 83.80,
        "Rb" => 85.4678,
        "Sr" => 87.62,
        "Y" => 88.9059,
        "Zr" => 91.22,
        "Nb" => 92.9064,
        "Mo" => 95.94,
        "Tc" => 98.0,
        "Ru" => 101.07,
        "Rh" => 102.9055,
        "Pd" => 106.4,
        "Ag" => 107.868,
        "Cd" => 112.41,
        "In" => 114.82,
        "Sn" => 118.69,
        "Sb" => 121.75,
        "I" => 126.9045,
        "Te" => 127.60,
        "Xe" => 131.30,
        "Cs" => 132.9054,
        "Ba" => 137.33,
        "La" => 138.9055,
        "Ce" => 140.12,
        "Pr" => 140.9077,
        "Nd" => 144.24,
        "Pm" => 145.0,
        "Sm" => 150.4,
        "Eu" => 151.96,
        "Gd" => 157.25,
        "Tb" => 158.9254,
        "Dy" => 162.50,
        "Ho" => 164.9304,
        "Er" => 167.26,
        "Tm" => 168.9342,
        "Yb" => 173.04,
        "Lu" => 174.967,
        "Hf" => 178.49,
        "Ta" => 180.9479,
        "W" => 183.85,
        "Re" => 186.207,
        "Os" => 190.2,
        "Ir" => 192.22,
        "Pt" => 195.09,
        "Au" => 196.9665,
        "Hg" => 200.59,
        "Tl" => 204.37,
        "Pb" => 207.2,
        "Bi" => 208.9804,
        "Po" => 209.0,
        "At" => 210.0,
        "Rn" => 222.0,
        "Fr" => 223.0,
        "Ra" => 226.0254,
        "Ac" => 227.0278,
        "Pa" => 231.0359,
        "Th" => 232.0381,
        "Np" => 237.0482,
        "U" => 238.029,
        "Pu" => 242.0,
        "Am" => 243.0,
        "Bk" => 247.0,
        "Cm" => 247.0,
        "No" => 250.0,
        "Cf" => 251.0,
        "Es" => 252.0,
        "Hs" => 277.0,
        "Mt" => 278.0,
        "Fm" => 257.0,
        "Md" => 258.0,
        "Lr" => 266.0,
        "Rf" => 267.0,
        "Bh" => 270.0,
        "Db" => 268.0,
        "Sg" => 269.0,
        "Ds" => 281.0,
        "Rg" => 282.0,
        "Cn" => 285.0,
        "Nh" => 286.0,
        "Fl" => 289.0,
        "Mc" => 290.0,
        "Lv" => 293.0,
        "Ts" => 294.0,
        "Og" => 294.0,
        _ => panic!("not an element"),
    }
}
