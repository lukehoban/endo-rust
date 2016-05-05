extern crate xi_rope;
extern crate image;
extern crate getopts;

mod dna;
mod rna;

use std::io::prelude::*;
use std::fs::File;
use xi_rope::Rope;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("l", "log-dna", "log DNA processing");    
    opts.optflag("i", "intermediate-rna", "render intermediate rna");
    opts.optopt("p", "page", "use prefix for rendering repair guide page #", "3");
    opts.optopt("g", "gene-table-page", "use prefix for rendering gene table page #", "3");
    opts.optopt("o", "out", "set output file name", "out.png");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let render_intermediates = matches.opt_present("i");
    let log_dna = matches.opt_present("l");
    let outpng = "out.png";
    let mut out_file = matches.opt_str("o").unwrap_or(String::from(outpng));
    let zero = "";
    let page = matches.opt_str("p").unwrap_or(String::from(zero)).parse::<u32>();
    let gene_table_page = matches.opt_str("g").unwrap_or(String::from(zero)).parse::<u32>();
    let mut prefix = matches.free.into_iter().next().unwrap_or(String::new());
    if prefix.len() > 0 && out_file == outpng {
        out_file = prefix.clone() + ".png";
    } else if let Ok(p) = page {
        let mut num = String::from("CCCCCCCCCCCCCCCCCCCCCCC").into_bytes();
        {
            let mut p = p;
            let mut i = 0;
            while p > 0 {
                if p % 2 == 1 {
                    num[i] = b'F';
                }
                p = p / 2;
                i += 1;
            }
        }
        prefix = format!("IIPIFFCPICFPPICIICCCCCCCCCCCCCCCCCCCCCCCCIICIPPP{}IIC", String::from_utf8(num).unwrap());
        out_file = format!("page{}-{}.png", p, prefix);
    } else if let Ok(g) = gene_table_page {
        let mut num = String::from("CCCCCCCCCCCCCCCCCCCCCCCIC").into_bytes();
        {
            let mut p = g - 1;
            let mut i = 0;
            while p > 0 {
                if p % 2 == 1 {
                    num[i] = b'F';
                }
                p = p / 2;
                i += 1;
            }
        }
        prefix = format!("IIPIFFCPICCFPICICFPPICICIPCCIIIIIICICPIICIIPIPIIICCPIICIICIPPP{}IICIIPIFFCPICFPPICIICCCCCCCCCCCCCCCCCCCCCCCCIICIPPPCFCFCFCCCCCCCCCCCCCCCCCIIC", String::from_utf8(num).unwrap());
        out_file = format!("genetable{}-{}.png", g, prefix);
    }
    
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    let endo = Rope::from(&s);
    
    let mut dna = Rope::from(prefix);
    dna.push(endo);
    
    let rna = dna::execute(dna, log_dna);
    println!("#RNA = {}", rna.len());
 
    rna::build(rna, &out_file, render_intermediates);
}