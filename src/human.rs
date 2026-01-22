

use alloc::{format, string::{String, ToString}};
use core::str;
use hashbrown::HashMap;

use crate::theory::*;

pub struct Dict {
    map: HashMap<String, ID>,
    rev: HashMap<ID, String>,
    i: ID
}
impl Dict {
    pub fn new() -> Self { Self {map: HashMap::new(), rev: HashMap::new(), i: 0} }
    pub fn get(&mut self, name: String) -> ID {
        match self.map.get(&name) {
            Some(id) => *id,
            None => {
                self.map.insert(name.clone(), self.i);
                self.rev.insert(self.i, name);
                let ret = self.i;
                self.i += 1;
                ret
            },
        }
    }
    /* expensive (kinda) */
    pub fn get_name(&self, id: ID) -> Option<String> {
        self.rev.get(&id).cloned()
    }
}

/* parse errors */
#[derive(Clone, Debug)]
pub struct PrsErr {
    pub msg: &'static str,
    pub byte: usize,
}

/* parser: binary s-expressions (s-pairs) */
struct Prsr<'a> {
    s: &'a [u8],
    i: usize,
}
impl<'a> Prsr<'a> {
    fn new(input: &'a str) -> Self {
        Self { s: input.as_bytes(), i: 0 }
    }

    fn eof(&self) -> bool { self.i >= self.s.len() }
    fn peek(&self) -> Option<u8> { self.s.get(self.i).copied() }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.i += 1;
        Some(b)
    }

    fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.i += 1,
                b';' => {
                    /* comment to end-of-line */
                    self.i += 1;
                    while let Some(c) = self.peek() {
                        self.i += 1;
                        if c == b'\n' { break; }
                    }
                }
                _ => break,
            }
        }
    }

    fn expect(&mut self, want: u8) -> Result<(), PrsErr> {
        self.skip_ws();
        match self.bump() {
            Some(got) if got == want => Ok(()),
            _ => Err(PrsErr { msg: "unexpected character", byte: self.i }),
        }
    }

    fn is_sym_char(b: u8) -> bool {
        matches!(
            b,
            b'a'..=b'z' |
            b'A'..=b'Z' |
            b'0'..=b'9' |
            b'_' | b'-' | b'+' | b'*' | b'/' | b'=' | b'<' | b'>' | b'!' | b'?' | b'.'
        )
    }

    fn parse_atom(&mut self) -> Result<String, PrsErr> {
        self.skip_ws();
        let start = self.i;

        while let Some(b) = self.peek() {
            if Self::is_sym_char(b) { self.i += 1; } else { break; }
        }

        if self.i == start {
            return Err(PrsErr { msg: "expected atom", byte: self.i });
        }

        match str::from_utf8(&self.s[start..self.i]) {
            Ok(sym) => Ok(sym.to_string()),
            Err(_) => Err(PrsErr { msg: "invalid utf-8 in symbol", byte: start }),
        }
    }

    fn parse_expr(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.skip_ws();
        match self.peek() {
            Some(b'(') => self.parse_blist(dict),
            Some(_) => {
                let sym = self.parse_atom()?;
                Ok(Kind::from(dict.get(sym)))
            }
            None => Err(PrsErr { msg: "unexpected end of input", byte: self.i }),
        }
    }

    // '(' expr expr ')', exactly two.
    fn parse_blist(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.expect(b'(')?;
        let l = self.parse_expr(dict)?;
        let r = self.parse_expr(dict)?;
        self.skip_ws();

        match self.peek() {
            Some(b')') => { self.i += 1; Ok(Kind::from((l, r))) }
            Some(_) => Err(PrsErr {
                msg: "binary list (s-pair) must contain exactly 2 binary s-expressions",
                byte: self.i
            }),
            None => Err(PrsErr { msg: "missing ')'", byte: self.i }),
        }
    }
}

pub fn parse(input: &str, dict: &mut Dict) -> Result<Kind, PrsErr> {
    let mut p = Prsr::new(input);
    let k = p.parse_blist(dict)?;
    p.skip_ws();
    if !p.eof() {
        return Err(PrsErr { msg: "trailing input", byte: p.i });
    }
    Ok(k)
}

pub fn unparse(root: &Kind, dict: &Dict) -> String {
    match root {
        Kind::Alp { id } => match dict.get_name(*id) {
            Some(name) => name,
            None => format!("#{root:?}"),
        },
        Kind::Zta { sid, .. } => match *sid {
            None => format!("#{root:?}"),
            Some(id) => match dict.get_name(id) {
                Some(name) => name,
                None => format!("#{root:?}"),
            },
        }
        Kind::Pir { l, r } => format!(
            "({} {})",
            unparse(l, dict), unparse(r, dict)
        )
    }
}
