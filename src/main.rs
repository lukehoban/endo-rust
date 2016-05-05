extern crate xi_rope;
extern crate image;
extern crate getopts;

mod dna;
mod rna;
mod gene;

use std::io::prelude::*;
use std::fs::File;
use xi_rope::Rope;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("l", "log-dna", "log DNA processing");    
    opts.optflag("t", "trace", "trace Fuun gene execution using RNA C*CC markers");    
    opts.optflag("x", "gene-table", "render the gene table contents");    
    opts.optflag("i", "intermediate-rna", "render intermediate rna");
    opts.optopt("p", "page", "use prefix for rendering repair guide page #", "3");
    opts.optopt("g", "gene-table-page", "use prefix for rendering gene table page #", "3");
    opts.optopt("z", "green-zone-section", "print the green zone section at the provided offset and length", "0x000510:0x00018");
    opts.optopt("o", "out", "set output file name", "out.png");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    let outpng = "out.png";
    
    let render_intermediates = matches.opt_present("i");
    let log_dna = matches.opt_present("l");
    let tracing = matches.opt_present("t");
    let show_gene_table = matches.opt_present("x");
    
    let page = matches.opt_str("p").unwrap_or(String::new()).parse::<u32>();
    let gene_table_page = matches.opt_str("g").unwrap_or(String::new()).parse::<u32>();
    let green_zone_section = matches.opt_str("z").map(|s| {
       let parse_number = |s: &str, default: usize| {
           if s.len() > 2 && &s[0..2] == "0x" {
                usize::from_str_radix(&s[2..], 16).unwrap_or(default)
           } else {
               s.parse::<usize>().unwrap_or(default)
           }
       };
       let parts = s.split(':').collect::<Vec<&str>>();
       let offset = parse_number(parts[0], 0);
       let length = parse_number(parts[1], 7509409);
       (offset, length)
    });
    let mut out_file = matches.opt_str("o").unwrap_or(String::from(outpng));
    let mut prefix = matches.free.into_iter().next().unwrap_or(String::new());

    // Update out_file and prefix based on other flags 
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
    
    // Get Endo
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    let endo = Rope::from(&s);

    if show_gene_table {
        println!("*** Gene Table ***");
        for gene in gene::gene_table().iter() {
            let bases = endo.clone().slice(13615 + gene.offset, 13615 + gene.offset + gene.length);
            println!("{:30} [{:8}:{:8}]: {}", gene.name, gene.offset, gene.length, dna::dna_to_string(&bases));
        }    
        println!("");
    }
    
    if let Some((offset, length)) = green_zone_section {
        // Green Zone starts at 13616
        let segment = endo.clone().slice(13615 + offset, 13615 + offset + length);
        println!("Green zone at offset {} of length {}:\n{}", offset, length, String::from(segment));
        return;
    }
    
    // Prepare DNA from Endo and prefix
    let mut dna = Rope::from(prefix);
    dna.push(endo);
    
    // Convert DNA -> RNA
    let rna = dna::execute(dna, log_dna, tracing);
    println!("#RNA = {}", rna.len());

    // Convert RNA -> Image(s)
    rna::build(rna, &out_file, render_intermediates);
}