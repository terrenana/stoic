use crate::matrix::GaussianElimination;
use crate::parser;
use crate::parser::ChemicalEquation;
use crate::parser::Side;
use na::DMatrix;
use num::rational::Ratio;
use num::{Integer, ToPrimitive};
use std::collections::HashMap;
use std::ops::Mul;

impl Mul<&Side> for Ratio<isize> {
    type Output = Ratio<isize>;
    fn mul(self, rhs: &Side) -> Self::Output {
        match rhs {
            Side::RHS => self * -1,
            Side::LHS => self,
        }
    }
}

pub struct Balancer;

impl Balancer {
    pub fn balance(equation: &str) -> Result<ChemicalEquation, String> {
        let chem_eq = parser::parse(equation)?;
        balance(chem_eq)
    }
    pub fn balance_real_time(equation: &str) -> String {
        let eq = parser::parse(equation).unwrap_or(ChemicalEquation::empty());
        if let Ok(bal_eq) = balance(eq.clone()) {
            bal_eq.to_string()
        } else {
            eq.to_string()
        }
    }
}

fn balance(eq: ChemicalEquation) -> Result<ChemicalEquation, String> {
    let mut elements = HashMap::new();
    for cpd in eq.terms.iter() {
        for elem in cpd.elements.keys() {
            elements.insert(elem.clone(), 0);
        }
    }
    elements = elements
        .iter_mut()
        .enumerate()
        .map(|(i, (val, _))| (val.clone(), i))
        .collect();
    let mut eq_matrix: DMatrix<Ratio<isize>> =
        DMatrix::from_element(elements.len(), eq.terms.len(), Ratio::new_raw(0, 1));
    let mut col = 0usize;
    for cpd in &eq.terms {
        for (elem, row) in elements.iter() {
            if let Some(coeff) = cpd.elements.get(elem) {
                *eq_matrix.get_mut((*row, col)).unwrap() =
                    Ratio::from_integer(*coeff as isize) * &cpd.side;
            }
        }
        col += 1;
    }

    let coeffs = GaussianElimination::new(eq_matrix)
        .solve()?
        .into_iter()
        .map(|v| {
            let lcm = v.iter().fold(1, |lcm, ratio| lcm.lcm(ratio.denom()));
            v.into_iter()
                .map(|ratio| lcm / *ratio.denom() * *ratio.numer())
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|int| int.to_usize().unwrap_or(1))
        .collect::<Vec<_>>();
    let mut eq = eq;
    for (i, cpd) in eq.terms.iter_mut().enumerate() {
        cpd.coefficient = coeffs[i].max(1);
    }
    Ok(eq)
}
