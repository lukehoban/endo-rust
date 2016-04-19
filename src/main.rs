use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;

enum PItem {
    Base(char),
    Skip(u64),
    Search(String),
    Open,
    Close
}

fn nat(chars: &mut Chars) -> u64 {
    34
}

fn consts(chars: &mut Chars) -> String {
    "ICFP".to_string()
}

fn pattern(dna: &str) -> Option<(&str, Vec<PItem>)> {
    let mut chars = dna.chars();
    //let mut rna = Vec::new<PItem>();
    let mut p = Vec::new();
    let mut dna = dna;
    loop {
        match chars.next() {
            Some('C') => p.push(PItem::Base('I')),
            Some('F') => p.push(PItem::Base('C')),
            Some('P') => p.push(PItem::Base('F')),
            Some('I') => match chars.next() {
                Some('C') => p.push(PItem::Base('P')),
                Some('P') => p.push(PItem::Skip(nat(&mut chars))),
                Some('F') => p.push(PItem::Search(consts(&mut chars))),
                Some('I') => match chars.next() {
                    Some('P') => panic!("nyi"),
                    Some('C') | Some('F') => return Some((dna, p)),
                    Some('I') => panic!("nyi"),
                    _ => return None     
                },
                _ => return None
            },        
            _ => return None,    
        }
    }
}

// fn template(dna: String) -> Option<(String, String)> {
//     ("", dna)
// }

// fn math_replace(p: String, t: String) -> Option


fn execute(dna: &str) -> Vec<String> {
    let rna = Vec::new();
    loop {
        match pattern(dna) {
            None => return rna,
            Some((p, dna)) => println!("Pattern: {}", p) 
        }
        // let (t, dna) = template(dna);
        // match_replace(p, t)
    }
}

fn main() {
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    
    println!("Endo bytes: {}", s.len());
    println!("Endo: {}", s.chars().take(10).collect::<String>());
    
    let rna = execute(s.as_str());
    
    println!("RNAs: {}", rna.len());
    
    let mut chars = s.chars();
    let _ = chars.next();
    let rest = chars.as_str();
    println!("{}", rest)
}
