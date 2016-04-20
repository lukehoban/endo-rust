use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;
use std::fmt;

enum PItem {
    Base(char),
    Skip(usize),
    Search(String),
    Open,
    Close
}

enum TItem {
    Base(char),
    Reference(usize, usize),
    Length(usize)
}

fn pattern_to_string(pat: &Vec<PItem>) -> String {
    pat.iter().map(|item| match item {
        &PItem::Base(c) => format!("{}", c),
        &PItem::Skip(n) => format!("!{}", n),
        &PItem::Search(ref s) => format!("?{}", s),
        &PItem::Open => "(".to_string(),
        &PItem::Close => ")".to_string()
    }).collect::<String>()
}

fn template_to_string(templ: &Vec<TItem>) -> String {
    templ.iter().map(|item| match item {
        &TItem::Base(c) => format!("{}", c),
        &TItem::Reference(n, l) => if l == 0 {
            format!("\\{}", n)
        } else {
            format!("\\{}({})", n, l)
        },
        &TItem::Length(n) => format!("|{}|", n)
    }).collect::<String>()
}

fn nat(chars: &mut Chars) -> Option<usize> {
    match chars.next() {
        Some('P') => Some(0usize),
        Some('I') | Some('F') => nat(chars).map(|n| 2*n),
        Some('C') => nat(chars).map(|n| 2*n + 1),
        _ => None,
    }
}

fn consts(chars: &mut Chars) -> String {
    let mut ret = String::from(""); 
    loop {
        match chars.next() {
            Some('C') => ret.push('I'),
            Some('F') => ret.push('C'),
            Some('P') => ret.push('F'),
            Some('I') => match chars.next() {
                Some('C') => ret.push('P'),
                _ => return ret.to_string()
            },
            _ => return ret.to_string()
        }
    }
}

fn pattern(dna: &str) -> Option<(&str, Vec<String>, Vec<PItem>)> {
    let mut chars = dna.chars();
    let mut rna = Vec::new();
    let mut p = Vec::new();
    let mut lvl = 0;
    loop {
        match chars.next() {
            Some('C') => p.push(PItem::Base('I')),
            Some('F') => p.push(PItem::Base('C')),
            Some('P') => p.push(PItem::Base('F')),
            Some('I') => match chars.next() {
                Some('C') => p.push(PItem::Base('P')),
                Some('P') => match nat(&mut chars) {
                    Some(n) => p.push(PItem::Skip(n)),     
                    None => return None
                },
                Some('F') => {
                    chars.next(); // three bases consumed!
                    let s = consts(&mut chars);
                    p.push(PItem::Search(s));
                },
                Some('I') => match chars.next() {
                    Some('P') => {
                        lvl = lvl + 1;
                        p.push(PItem::Open);
                    },
                    Some('C') | Some('F') => {
                        if lvl == 0 {
                            return Some((chars.as_str(), rna, p)); 
                        } else {
                            lvl = lvl - 1;
                            p.push(PItem::Close); 
                        }
                    },
                    Some('I') => rna.push(chars.by_ref().take(7).collect::<String>()),
                    _ => return None
                },
                _ => return None
            },        
            _ => return None
        }
    }
}

fn template(dna: &str) -> Option<(&str, Vec<String>, Vec<TItem>)> {
    let mut chars = dna.chars();
    let mut rna = Vec::new();
    let mut t = Vec::new();
    loop {
        match chars.next() {
            Some('C') => t.push(TItem::Base('I')),
            Some('F') => t.push(TItem::Base('C')),
            Some('P') => t.push(TItem::Base('F')),
            Some('I') => match chars.next() {
                Some('C') => t.push(TItem::Base('P')),
                Some('F') | Some('P') => match nat(&mut chars) {
                    Some(l) => match nat(&mut chars) {
                        Some(n) => t.push(TItem::Reference(n, l)),
                        None => return None
                    },
                    None => return None
                },
                Some('I') => match chars.next() {
                    Some('C') | Some('F') => return Some((chars.as_str(), rna, t)),
                    Some('P') => match nat(&mut chars) {
                        Some(n) => t.push(TItem::Length(n)),
                        None => return None
                    },
                    Some('I') => rna.push(chars.by_ref().take(7).collect::<String>()),
                    _ => return None
                },
                _ => return None
            },
            _ => return None
        }
    }
}

fn match_replace(p: Vec<PItem>, t: Vec<TItem>, dna: &str) -> Option<String> {
    let mut i = 0usize;
    let mut e = Vec::new();
    let mut c = Vec::new();
    for item in p {
        match item {
            PItem::Base(c) => match dna.bytes().nth(i) {
                Some(ch) if (ch as u32) == (c as u32) => i = i + 1,
                _ => return None
            },
            PItem::Skip(n) => {
                i = i + n;
                if i  > dna.len() {
                    return None
                }
            },
            PItem::Search(ref s) => panic!("nyi"),
            PItem::Open => c.push(i),
            PItem::Close => match c.pop()  {
                Some(c0) => e.push(&dna[c0..i]),
                None => return None
            }
        }
    }
    println!("succesful match of length {}", i);
    for (i, captured) in e.iter().enumerate() {
        println!("e[{}] = {}", i, dna_to_string(captured));
    }
    return Some(replace(t, e, &dna[i..]));
}

fn replace<'a>(t: Vec<TItem>, e: Vec<&'a str>, dna: &'a str) -> String {
    let mut r = "".to_owned();
    for item in t {
         match item {
             TItem::Base(c) => r.push(c),
             TItem::Reference(n, l) => r.push_str(protect(l, e[n])),
             TItem::Length(n) => panic!("nyi")
         }
    }
    r.push_str(dna);
    r
}

fn protect(l: usize, d: &str) -> &str {
    if l == 0 {
        d
    } else {
        panic!("nyi - protect")
    }
}

fn dna_to_string(dna: &str) -> String {
   let mut s = dna.chars().take(10).collect::<String>();
   if dna.len() > 10 {
       s = s + "...";
   }
   s = s + " (" + &dna.len().to_string() + " bases)";
   s
}

fn execute(dna: &str) -> Vec<String> {
    let mut dna = dna;
    let mut rna = Vec::new();
    let mut iteration = 0;
    loop {
        iteration = iteration + 1;
        println!("");
        println!("iteration = {}", iteration);
        println!("dna = {}", dna_to_string(dna));
        match pattern(dna) {
            None => return rna,
            Some((dna2, rna2, p)) => {
                println!("pattern  {}", pattern_to_string(&p));
                rna.extend(rna2.into_iter());
                dna = dna2;
                match template(dna) {
                    None => return rna,
                    Some((dna3, rna3, t)) => {
                        println!("template {}", template_to_string(&t));
                        dna = dna3;
                        rna.extend(rna3.into_iter());
                        match match_replace(p, t, dna) {
                            Some(dna4) => dna = &dna4,
                            Nonw => ()
                        }
                    }
                }
            } 
        }
        
    }
}

fn main() {
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    
    println!("Endo bytes: {}", s.len());
    println!("Endo: {}", dna_to_string(&s));
    
    let rna = execute(s.as_str());
    
    println!("RNAs: {}", rna.len());
    
    let mut chars = s.chars();
    let _ = chars.next();
    let rest = chars.as_str();
}
