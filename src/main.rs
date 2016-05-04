extern crate xi_rope;
mod dna;

use std::io::prelude::*;
use std::fs::File;
use xi_rope::Rope;

fn main() {
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    let dna = Rope::from(&s);
    
    let rna = dna::execute(dna);

    println!("RNAs: {}", rna.len());
}