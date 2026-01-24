

use alloc::{format, rc::Rc, string::String};
use core::str;
use hashbrown::HashMap;

use crate::theory::*;

pub struct Dict {
    map: HashMap< Rc<String>, ID>,
    rev: HashMap<ID, Rc<String>>,
    i: ID
}
impl Dict {
    pub fn new() -> Self { Self {map: HashMap::new(), rev: HashMap::new(), i: 0} }
    pub fn get(&mut self, name: String) -> Option<ID> {
        match self.map.get(&name) {
            Some(id) => Some(*id),
            None => {
                let owd: Rc<String> = Rc::from(name);
                self.map.insert(owd.clone(), self.i);
                self.rev.insert(self.i, owd);

                let ret = self.i;
                match self.i.checked_add(1) {
                    Some(ni) => {
                        self.i = ni;
                        Some(ret)
                    },
                    None => None,
                }
            },
        }
    }
    /* expensive (kinda) */
    pub fn get_name(&self, id: ID) -> Option<&str> {
        self.rev.get(&id).map(Rc::as_ref).map(|x| x.as_str())
    }
}

/* parse errors */
#[derive(Clone, Debug)]
pub struct PrsErr {
    pub msg: &'static str,
    pub pos: usize,
}

/* parser: binary s-expressions (s-pairs) */
struct Prsr<'a> {
    s: &'a str,
    i: usize,
}
impl<'a> Prsr<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, i: 0 }
    }

    fn eof(&self) -> bool { self.i >= self.s.chars().count() }
    fn peek(&self) -> Option<char> { self.s.chars().nth(self.i) }

    fn bump(&mut self) -> Option<char> {
        let b = self.peek()?;
        self.i += 1;
        Some(b)
    }

    fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                ' ' | '\t' | '\r' | '\n' => self.i += 1,
                ';' => {
                    /* comment to end-of-line */
                    self.i += 1;
                    while let Some(c) = self.peek() {
                        self.i += 1;
                        if c == '\n' { break; }
                    }
                }
                _ => break,
            }
        }
    }

    fn expect(&mut self, want: char) -> Result<(), PrsErr> {
        self.skip_ws();
        match self.bump() {
            Some(got) if got == want => Ok(()),
            _ => Err(PrsErr {msg: "unexpected character", pos: self.i}),
        }
    }

    fn is_sym_char(b: char) -> bool {
        !matches!(b, ' ' | '\t' | '\r' | '\n' | '(' | ')' | ';')
    }

    fn parse_atom(&mut self) -> Result<String, PrsErr> {
        self.skip_ws();
        let start = self.i;

        while let Some(b) = self.peek() {
            if Self::is_sym_char(b) { self.i += 1; } else { break; }
        }

        if self.i == start {
            return Err(PrsErr {msg: "expected atom", pos: self.i});
        }

        Ok(self.s.chars().skip(start).take(self.i - start).collect())
    }

    fn parse_expr(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.skip_ws();
        match self.peek() {
            Some('(') => self.parse_blist(dict),
            Some(_) => {
                let sym = self.parse_atom()?;
                match dict.get(sym) {
                    Some(gsy) => Ok(Kind::from(gsy)),
                    None => Err(PrsErr {msg: "internal namespace full", pos: self.i}),
                }
            }
            None => Err(PrsErr {msg: "unexpected end of input", pos: self.i}),
        }
    }

    // '(' expr expr ')', exactly two.
    fn parse_blist(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.expect('(')?;
        let l = self.parse_expr(dict)?;
        let r = self.parse_expr(dict)?;
        self.skip_ws();

        match self.peek() {
            Some(')') => { self.i += 1; Ok(Kind::from((l, r))) }
            Some(_) => Err(PrsErr {
                msg: "s-pair must contain exactly 2 binary s-pairs",
                pos: self.i
            }),
            None => Err(PrsErr {msg: "missing ')'", pos: self.i}),
        }
    }
}

pub fn parse(input: &str, dict: &mut Dict) -> Result<Kind, PrsErr> {
    let mut p = Prsr::new(input);
    let k = p.parse_blist(dict)?;
    p.skip_ws();
    if !p.eof() {
        return Err(PrsErr {msg: "trailing input", pos: p.i});
    }
    Ok(k)
}

pub fn unparse(root: &Kind, dict: &Dict) -> String {
    match root {
        Kind::Alp { id } => match dict.get_name(*id) {
            Some(name) => name.into(),
            None => format!("#{root:?}"),
        },
        Kind::Zta { sid, .. } => match *sid {
            None => format!("#{root:?}"),
            Some(id) => match dict.get_name(id) {
                Some(name) => name.into(),
                None => format!("#{root:?}"),
            },
        }
        Kind::Pir { l, r } => format!(
            "({} {})",
            unparse(l, dict), unparse(r, dict)
        )
    }
}
