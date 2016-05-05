use std::str::Chars;
use xi_rope::{Rope, ChunkIter};

struct RopeCharIter<'a> {
    chunk_iter: ChunkIter<'a>,
    char_iter: Option<Chars<'a>>,
    buf: Vec<char>,
    buf_index: usize,
    index: usize
}

impl<'a> Iterator for RopeCharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.buf_index = 0;
        self.index = self.index + 1;
        if self.buf.is_empty() {
            self.next_inner()
        } else {
            Some(self.buf.remove(0))
        }
    }
}

impl<'a> RopeCharIter<'a> {
    fn next_inner(&mut self) -> Option<char> {
        let next_char_iter = match self.char_iter {
                Some(ref mut chars) => match chars.next() {
                    Some(c) => {  
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
            self.next_inner()
    }
    
    fn peek(&mut self) -> Option<&char> {
        let ret = if self.buf_index < self.buf.len() {
            Some(&self.buf[self.buf_index])
        } else {
            match self.next_inner() {
                Some(x) => {
                    self.buf.push(x);
                    Some(&self.buf[self.buf_index])
                }
                None => return None,
            }
        };

        self.buf_index += 1;
        ret
    }
}

fn rope_char_iter(rope: &Rope) -> RopeCharIter {
    RopeCharIter {
        chunk_iter: rope.iter_chunks(),
        char_iter: None,
        buf: Vec::new(),
        buf_index: 0,
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
        &PItem::Search(ref s) => format!("?\"{}\"", s),
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

fn consts(iter: &mut RopeCharIter) -> String {
    let mut ret = String::from("");
    loop {
        match iter.peek() {
            Some(&'C') => { ret.push('I'); iter.next(); },
            Some(&'F') => { ret.push('C'); iter.next(); },
            Some(&'P') => { ret.push('F'); iter.next(); },
            Some(&'I') => {
                match iter.peek() {
                    Some(&'C') => {
                        iter.next();
                        iter.next();
                        ret.push('P')
                    },
                    _ => return ret.to_string()
                }
            },
            _ => return ret.to_string()
        };
        
    }
}

#[test]
fn consts_test() {
    let input = "CPICCFPICICFCPPIIC";
    let rope = Rope::from(input);
    let mut iter = rope_char_iter(&rope);
    assert_eq!("IFPICFPPCIFF", consts(&mut iter));    
    assert_eq!("IIC", &String::from(rope.clone().slice(iter.index, rope.len())));    
}

fn pattern(chars: &mut RopeCharIter) -> Option<(Vec<String>, Vec<PItem>)> {
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
                Some('P') => match nat(chars) {
                    Some(n) => p.push(PItem::Skip(n)),     
                    None => return None
                },
                Some('F') => {
                    chars.next(); // three bases consumed!
                    let s = consts(chars);
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
                    Some('I') => rna.push(chars.take(7).collect::<String>()),
                    _ => return None
                },
                _ => return None
            },        
            _ => return None
        }
    }
}

fn template(chars: &mut RopeCharIter) -> Option<(Vec<String>, Vec<TItem>)> {
    let mut rna = Vec::new();
    let mut t = Vec::new();
    loop {
        match chars.next() {
            Some('C') => t.push(TItem::Base('I')),
            Some('F') => t.push(TItem::Base('C')),
            Some('P') => t.push(TItem::Base('F')),
            Some('I') => match chars.next() {
                Some('C') => t.push(TItem::Base('P')),
                Some('F') | Some('P') => match nat(chars) {
                    Some(l) => match nat(chars) {
                        Some(n) => t.push(TItem::Reference(n, l)),
                        None => return None
                    },
                    None => return None
                },
                Some('I') => match chars.next() {
                    Some('C') | Some('F') => return Some((rna, t)),
                    Some('P') => match nat(chars) {
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

fn match_replace(p: Vec<PItem>, t: Vec<TItem>, dna: Rope, logging: bool) -> Rope {
    let mut i = 0usize;
    let mut e = Vec::new();
    let mut c = Vec::new();
    let mut failed = false;
    for item in p {
        match item {
            PItem::Base(c) => { 
                if dna.byte_at(i) == c as u8 {
                    i = i + 1
                } else {
                    failed = true
                }
            },
            PItem::Skip(n) => {
                i = i + n;
                if i > dna.len() {
                    failed = true
                }
            },
            PItem::Search(ref s) => {
                match search(i, s, &dna) {
                    Some(n) => i = n,
                    None => failed = true
                }
            },
            PItem::Open => c.push(i),
            PItem::Close => match c.pop()  {
                Some(c0) => e.push(dna.clone().slice(c0, i)),
                None => failed = true
            }
        }
        if failed {
            if logging { println!("failed match") }
            return dna    
        }
    }
    if logging {
        println!("succesful match of length {}", i);
        for (i, captured) in e.iter().enumerate() {
           println!("e[{}] = {}", i, dna_to_string(&captured));
        }
    }
    let dna_len = dna.len();
    replace(t, e, dna.slice(i, dna_len))
}

fn search(i: usize, s: &str, dna: &Rope) -> Option<usize> {
    let mut n = 0;
    let rest_dna = dna.clone().slice(i, dna.len());
    let mut iter = rope_char_iter(&rest_dna);
    loop {
        let mut s_iter = s.chars();
        loop {
            let schar = s_iter.next();
            let dnachar = iter.peek();
            match (schar, dnachar) {
                (None, _) => return Some(n + i + s.len()),
                (Some(sc), Some(dc)) if sc == *dc => continue,
                _ => {
                    break;
                }  
            }
        }
        n = n + 1;
        match iter.next() {
            None => return None,
            Some(c) => continue
        }
    }
}

fn env_get(e: &Vec<Rope>, i: usize) -> Rope  {
    if i >= e.len() {
        Rope::from("")
    } else {
        e[i].clone()
    } 
}

fn replace(t: Vec<TItem>, e: Vec<Rope>, dna: Rope) -> Rope {
    let mut ret = Rope::from("");
    let mut bases = String::new();
    for item in t {
        match item {
            TItem::Base(c) => bases.push_str(&c.to_string()),
            TItem::Reference(n, l) => {
                if bases.len() > 0 {
                    ret.push_str(&bases);
                    bases = String::new();
                }
                ret.push(protect(l, env_get(&e, n)))
            },
            TItem::Length(n) => {
                if bases.len() > 0 {
                    ret.push_str(&bases);
                    bases = String::new();
                }
                ret.push(asnat(env_get(&e, n).len()))
            }
        }
    }
    if bases.len() > 0 {
        ret.push_str(&bases);
    }
    ret.push(dna);
    ret
}

fn protect(l: usize, d: Rope) -> Rope {
    if l == 0 {
        d
    } else {
        protect(l - 1, quote(d))
    }
}

fn quote(d: Rope) -> Rope {
    let mut s = String::new();
    for c in rope_char_iter(&d) {
        match c {
            'I' => s = s + "C",
            'C' => s = s + "F",
            'F' => s = s + "P",
            'P' => s = s + "IC",
            _ => return Rope::from(s)
        }
    }
    Rope::from(s)
}

fn asnat(mut n: usize) -> Rope {
    let mut s = String::new();
    loop {
        if n == 0 {
            s = s + "P";
            return Rope::from(s);
        }
        s = s + (match n % 2 { 0 => "I", _ => "C" });
        n = n / 2;
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

pub fn execute(mut dna: Rope, logging: bool) -> Vec<String> {
    let mut rna = Vec::new();
    let mut iteration = -1  ;
    loop {
        iteration = iteration + 1;
        if iteration % 10000 == 0 {
            println!("iteration = {}", iteration);
        
        }
        if logging {
            println!("");
            println!("iteration = {}", iteration);
            println!("dna = {}", dna_to_string(&dna));
        }
        let (p, t, index) = {
            let mut chars = rope_char_iter(&dna);
            let (rna2, p) = match pattern(&mut chars) {
                None => return rna,
                Some((rna2, p)) => (rna2, p)
            };
            if logging {
                println!("pattern  {}", pattern_to_string(&p));
            }
            rna.extend(rna2.into_iter());
            let (rna3, t) = match template(&mut chars) {
                None => return rna,
                Some((rna3, t)) => (rna3, t)
            };
            if logging {
                println!("template {}", template_to_string(&t));
            }
            rna.extend(rna3.into_iter());
            if logging {
                println!("len(pattern + template) = {}", chars.index);
            }
            (p, t, chars.index)
        };
        let dna_len = dna.len();
        dna = match_replace(p, t, dna.slice(index, dna_len), logging);
        if logging {
            println!("len(rna) = {}", rna.len());
        }
    }
}