extern crate nalgebra as na;

mod balance;
mod matrix;
mod parser;

use crate::balance::Balancer;
use std::io::stdin;

fn main() -> Result<(), String> {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("unable");
        println!("{}", Balancer::balance(&input));
    }
}
