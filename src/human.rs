

use alloc::{format, rc::Rc, string::String};
use core::{fmt::Display, iter::Peekable, str};
use hashbrown::HashMap;
use thiserror::Error;

use crate::theory::{ID, Kind};

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
#[derive(Debug, Error)]
pub enum ParserErr {
    #[error("expected '{what}' at {pos}")]
    Expected { what: char, pos: usize },

    #[error("expected an atom at {pos}")]
    NeedAtom { pos: usize },

    #[error("namespace full while at {pos}")]
    NamespaceFull { pos: usize },

    #[error("end of input at {pos}")]
    EndOfInput { pos: usize },

    #[error("pair has more than 2 members at {pos}")]
    PairHasMore { pos: usize },
}
type PrsErr = ParserErr;

struct Prsable<It: Iterator<Item = char>> {
    inner: Peekable<It>,
    pos: usize
}
impl<It: Iterator<Item = char>> Prsable<It> {
    fn new(it: It) -> Self { Self { inner: it.peekable(), pos: 0 }}
    fn next(&mut self) -> Option<char> {
        let n = self.inner.next()?;
        self.pos += 1;
        Some(n)
    }
    fn peek(&mut self) -> Option<char> { self.inner.peek().copied() }
}

/* parser: binary s-expressions (s-pairs) */
pub struct Parser<It: Iterator<Item = char>> {
    it: Prsable<It>
}

type Prsr<It> = Parser<It>;
impl<It: Iterator<Item = char>> Prsr<It> {
    pub fn new(it: It) -> Self { Self { it: Prsable::new(it) }}

    fn skip_ws(&mut self) {
        while let Some(b) = self.it.peek() {
            match b {
                ' ' | '\t' | '\r' | '\n' => { self.it.next(); },
                ';' => { /* comment to end-of-line */
                    while let Some(c) = self.it.next() {
                        if c == '\n' {
                            self.it.next(); /* eat \n */
                            break
                        }
                    }
                }
                _ => break,
            }
        }
    }

    fn expect(&mut self, want: char) -> Result<(), PrsErr> {
        self.skip_ws();
        match self.it.next() {
            Some(got) if got == want => Ok(()),
            _ => Err(PrsErr::Expected { what: want, pos: self.it.pos }),
        }
    }

    fn is_sym_char(b: char) -> bool {
        !matches!(b, ' ' | '\t' | '\r' | '\n' | '(' | ')' | ';')
    }

    pub fn parse_atom(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.skip_ws();
        let mut st = String::new();
        let start = self.it.pos;

        while let Some(b) = self.it.peek() {
            if Self::is_sym_char(b) {
                self.it.next(); /* consume */
                st.push(b);
            } else { break; }
        }

        if self.it.pos == start {
            return Err(PrsErr::NeedAtom { pos: self.it.pos });
        }

        match dict.get(st) {
            Some(sy) => Ok(Kind::from(sy)),
            None => Err(PrsErr::NamespaceFull { pos: self.it.pos }),
        }
    }

    pub fn parse_expr(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.skip_ws();
        match self.it.peek() {
            Some('(') => self.parse_spair(dict),
            Some(_) => self.parse_atom(dict),
            None => Err(PrsErr::EndOfInput { pos: self.it.pos }),
        }
    }

    // '(' expr expr ')', exactly two.
    pub fn parse_spair(&mut self, dict: &mut Dict) -> Result<Kind, PrsErr> {
        self.expect('(')?;
        let l = self.parse_expr(dict)?;
        let r = self.parse_expr(dict)?;
        self.skip_ws();

        match self.it.peek() {
            Some(')') => {
                self.it.next(); /* consume */
                Ok(Kind::from((l, r)))
            }
            Some(_) => Err(PrsErr::PairHasMore { pos: self.it.pos }),
            None => Err(PrsErr::Expected { what: ')', pos: self.it.pos }),
        }
    }
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

pub struct View<'a> {
    root: &'a Kind,
    dict: &'a Dict
}
impl<'a> View<'a> {
    pub fn new(root: &'a Kind, dict: &'a Dict) -> Self { Self { root, dict } }
}

impl<'a> Display for View<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.root {
            Kind::Alp { id } => match self.dict.get_name(*id) {
                Some(name) => write!(f, "{name}"),
                None => write!(f, "#{:?}", self.root),
            },
            Kind::Zta { sid, .. } => match *sid {
                None => write!(f, "#{:?}", self.root),
                Some(id) => match self.dict.get_name(id) {
                    Some(name) => write!(f, "{name}"),
                    None => write!(f, "#{:?}", self.root),
                },
            }
            Kind::Pir { l, r } => write!(
                f, "({} {})",
                View::new(l, self.dict),
                View::new(r, self.dict)
            )
        }
    }
}
