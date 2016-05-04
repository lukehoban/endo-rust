extern crate xi_rope;
extern crate image;
extern crate getopts;

mod dna;
mod rna;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use xi_rope::Rope;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("o", "out", "set output file name", "out.png");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let outpng = "out.png";
    let mut out_file = matches.opt_str("o").unwrap_or(String::from(outpng));
    let prefix = matches.free.into_iter().next().unwrap_or(String::new());
    if prefix.len() > 0 && out_file == outpng {
        out_file = prefix.clone() + ".png";
    }
    
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    let endo = Rope::from(&s);
    
    let mut dna = Rope::from(prefix);
    dna.push(endo);
    
    let rna = dna::execute(dna);
    println!("RNAs: {}", rna.len());
 
    let bitmap = rna::build(rna);
 
    let _ = bitmap.save(&Path::new(&out_file)).unwrap();
}