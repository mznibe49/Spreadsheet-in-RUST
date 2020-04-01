#[macro_use] extern crate lazy_static;
use std::env;

pub mod cell;
pub mod spreadsheet;
pub mod rectangle;
pub mod coordinates;
pub mod check;
pub mod rules;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    spreadsheet::SpreadSheet::process(&args).unwrap();
}
