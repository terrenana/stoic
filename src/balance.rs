use std::collections::{HashMap};
use crate::parser::Equation;
use na::{DMatrix};
use num::{Integer, ToPrimitive};
use num::rational::Ratio;
use crate::g_elim::GaussianElimination;

pub fn balance(mut eq: Equation) -> Vec<usize> {
    let mut elements = HashMap::new();
    for cpd in eq.rhs.iter() {
        for elem in cpd.elements.keys() {
            elements.insert(elem.clone(), 0);
        }
    }
    elements = elements.iter_mut().enumerate().map(|(i, (val, size))| (val.clone(), i) ).collect();
    for cpd in &eq.lhs {
        for elem in cpd.elements.keys() {
            if let None = elements.get(elem) {
                panic!("element existed on lhs that did not exist on rhs");
            }
        }
    }
    let mut eq_matrix: DMatrix<Ratio<isize>> = DMatrix::from_element(elements.len(), eq.lhs.len() + eq.rhs.len(), Ratio::new_raw(0, 1));

        let mut col = 0usize;
        for cpd in &eq.lhs {
            for (elem, row) in elements.iter() {
                if let Some(coeff) = cpd.elements.get(elem) {
                    *eq_matrix.get_mut((*row, col)).unwrap() = Ratio::from_integer(*coeff as isize);
                }
            }
            col += 1;
        }
        for cpd in &eq.rhs {
            for (elem, row) in elements.iter() {
                if let Some(coeff) = cpd.elements.get(elem) {
                    *eq_matrix.get_mut((*row, col)).unwrap() = Ratio::from_integer(*coeff as isize * -1);
                }
            }
            col += 1;
        }
    GaussianElimination::new(eq_matrix).solve().into_iter()
        .map(|v| {
            let lcm = v.iter().fold(1, |lcm, ratio| lcm.lcm(ratio.denom()));
            v.into_iter()
                .map(|ratio| lcm / *ratio.denom() * *ratio.numer()).collect::<Vec<_>>()
        })
        .flatten().map(|int| int.to_usize().unwrap()).collect::<Vec<_>>()
}