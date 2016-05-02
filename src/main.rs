extern crate xi_rope;

use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;
use std::fmt;

use xi_rope::{Rope, ChunkIter};

struct RopeCharIter<'a> {
    chunk_iter: ChunkIter<'a>,
    char_iter: Option<Chars<'a>>,
    index: usize
}

impl<'a> Iterator for RopeCharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let next_char_iter = match self.char_iter {
            Some(ref mut chars) => match chars.next() {
                Some(c) => {
                    self.index = self.index + 1;  
                    return Some(c)
                },
                None => None
            },
            None => match self.chunk_iter.next() {
                Some(next_chunk) => Some(next_chunk.chars()), 
                None => return None
            }
        };
        self.char_iter = next_char_iter;
        self.next()
    }
}

fn rope_char_iter(rope: &Rope) -> RopeCharIter {
    RopeCharIter {
        chunk_iter: rope.iter_chunks(),
        char_iter: None,
        index: 0
    }    
}

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

fn nat(chars: &mut RopeCharIter) -> Option<usize> {
    match chars.next() {
        Some('P') => Some(0usize),
        Some('I') | Some('F') => nat(chars).map(|n| 2*n),
        Some('C') => nat(chars).map(|n| 2*n + 1),
        _ => None,
    }
}

fn consts(chars: &mut RopeCharIter) -> String {
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

fn pattern(mut chars: &mut RopeCharIter) -> Option<(Vec<String>, Vec<PItem>)> {
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
                            return Some((rna, p)); 
                        } else {
                            lvl = lvl - 1;
                            p.push(PItem::Close); 
                        }
                    },
                    Some('I') => {
                        rna.push(chars.take(7).collect::<String>());
                        println!("rna.len() = {}", rna.len())
                    },
                    _ => return None
                },
                _ => return None
            },        
            _ => return None
        }
    }
}

fn template(mut chars: &mut RopeCharIter) -> Option<(Vec<String>, Vec<TItem>)> {
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
                    Some('C') | Some('F') => return Some((rna, t)),
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
        let captured = Rope::from(captured);
        println!("e[{}] = {}", i, dna_to_string(&captured));
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

fn dna_to_string(dna: &Rope) -> String {
   let mut s = rope_char_iter(dna).take(10).collect::<String>();
   if dna.len() > 10 {
       s = s + "...";
   }
   s = s + " (" + &dna.len().to_string() + " bases)";
   s
}

fn execute(mut dna: Rope) -> Vec<String> {
    let mut rna = Vec::new();
    let mut iteration = -1  ;
    loop {
        iteration = iteration + 1;
        println!("");
        println!("iteration = {}", iteration);
        println!("dna = {}", dna_to_string(&dna));
        let (p, t, index) = {
            let mut chars = rope_char_iter(&dna);
            let (rna2, p) = match pattern(&mut chars) {
                None => return rna,
                Some((rna2, p)) => (rna2, p)
            };
            println!("pattern  {}", pattern_to_string(&p));
            rna.extend(rna2.into_iter());
            let (rna3, t) = match template(&mut chars) {
                None => return rna,
                Some((rna3, t)) => (rna3, t)
            };
            println!("template {}", template_to_string(&t));
            rna.extend(rna3.into_iter());
            (p, t, chars.index)
        };
        
        println!("index = {}", index);
        
        let restdna = dna.clone().slice(index, dna.len());
        dna = match match_replace(p, t, &String::from(&restdna)) {
            Some(dna4) => Rope::from(&dna4),
            None => continue 
        } 
    }
}

fn main() {
    let mut f = File::open("endo.dna").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s); 
    let dna = Rope::from(&s);
    
    {
    let mut iter = rope_char_iter(&dna);
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    if let Some(c) = iter.next() { println!("{}", c) };
    }
    
    println!("Endo bytes: {}", s.len());
    println!("Endo: {}", dna_to_string(&dna));
    
    let rna = execute(dna);

    println!("RNAs: {}", rna.len());
}
