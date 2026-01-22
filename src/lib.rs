/* written by hand - by Hans - with love */
/* BSD-3 License */

#![no_std]

extern crate alloc;
extern crate core;

use core::{fmt, mem};
use alloc::{boxed::Box, string::String};
use hashbrown::HashMap;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[allow(clippy::empty_line_after_outer_attr)]

/******************************************************************/
/* remember to always give back what you dont use */

pub type ID = usize;
pub type ZtaFn = for<'a> fn(inp: &'a mut Kind) -> Result<Kind, &'a mut Kind>;

#[derive(Clone)]
pub enum Kind {
    Alp {id: ID},
    Zta {sid: Option<ID>, hid: ID, fnc: ZtaFn},
    Pir {l: Box<Kind>, r: Box<Kind>}
}
impl From<ID> for Kind {
    fn from(val: ID) -> Self {
        Self::Alp {id: val}
    }
}
impl TryFrom<(ZtaFn, ID)> for Kind {
    type Error = ();
    fn try_from(val: (ZtaFn, ID)) -> Result<Self, Self::Error> {
        if val.1 < 2 { return Err(()) }
        Ok(Self::Zta {sid: None, hid: val.1, fnc: val.0})
    }
}
impl From<(Kind, Kind)> for Kind {
    fn from(val: (Kind, Kind)) -> Self {
        Self::Pir {l: Box::new(val.0), r: Box::new(val.1)}
    }
}
impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Kind::Alp { id } => write!(f, "I:{id}"),
            Kind::Zta { sid, hid, ..} => match sid {
                Some(id) => write!(f, "Z:{hid}:{id}"),
                None => write!(f, "Z:{hid}"),
            },
            Kind::Pir { l, r } => write!(f, "({l:?} {r:?})"),
        }
    }
}

/******************************************************************/
/* remember to always give back what you dont use */

struct MapIDK {
    map: HashMap<ID, Option<Kind>>
}
impl MapIDK {
    fn new() -> Self { Self {map: HashMap::new()} }
    fn get(&mut self, id: ID) -> Option<Kind> {
        self.map.get_mut(&id).and_then(|k| k.take())
    }
    fn set(&mut self, id: ID, v: Kind) {
        self.map.insert(id, Some(v));
    }
}

/******************************************************************/
/* remember to always give back what you dont use */

struct Mtc {
    map: MapIDK
}
impl Mtc {
    fn new() -> Self { Self { map: MapIDK::new() } }
    fn alp(&mut self, x: &mut Kind, n: &mut Kind) -> bool {
        match n {
            Kind::Alp { id } => match self.map.get(*id) {
                Some(mut ol) => {
                    if self.mtc(x, &mut ol) {
                        self.map.set(*id, ol); /* re-insert */
                        return true
                    }
                    /* ol stays removed */
                    false
                },
                None => {
                    let mut k = x.clone();
                    if let Kind::Zta { sid, ..} = &mut k {
                        *sid = Some(*id);
                    }

                    self.map.set(*id, k);
                    true
                },
            },
            _ => false
        }
    }
    fn zta(&mut self, x: &mut Kind, n: &mut Kind) -> bool {
        match x {
            Kind::Pir { l: mz, r: inp } => match mz.as_mut() {
                Kind::Zta {fnc, ..}
                => match fnc(inp) {
                    Ok(xr) => {
                        *x = xr;
                        self.mtc(x, n)
                    },
                    Err(_) => false,
                },
                _ => false
            },
            _ => false
        }
    }
    fn rec(&mut self, x: &mut Kind, n: &mut Kind) -> bool {
        match (x, n) {
            (
                Kind::Zta {hid: xid, ..},
                Kind::Zta {hid: nid, ..}
            ) => {xid == nid},
            (
                Kind::Pir {l: xl, r: xr},
                Kind::Pir {l: nl, r: nr}
            ) => {
                if !self.mtc(xl, nl) {return false}
                self.mtc(xr, nr)
            },
            _ => false
        }
    }
    fn mtc(&mut self, x: &mut Kind, n: &mut Kind) -> bool {
        if self.alp(x, n) {return true}
        if self.rec(x, n) {return true}
        self.zta(x, n)
    }
    fn ins(&mut self, b: &mut Kind) {
        match b {
            Kind::Alp {id} => match self.map.get(*id) {
                Some(k) => {
                    self.map.set(*id, k.clone()); /* replace for next need */
                    *b = k
                },
                None => (),
            },
            Kind::Pir {l, r} => {
                self.ins(l);
                self.ins(r)
            },
            _ => ()
        }
    }
}

/******************************************************************/
/* remember to always give back what you dont use */

pub fn eta<'a>(inp: &'a mut Kind) -> Result<Kind, &'a mut Kind> {
    let (n, b, x) = match inp {
        Kind::Pir {l: nb, r: x} => {
            match nb.as_mut() {
                Kind::Pir {l: n, r: b} => (n, b, x),
                _ => return Err(inp)
            }
        },
        _ => return Err(inp)
    };

    let mut mt = Mtc::new();
    if !mt.mtc(x, n) {return Err(inp)}
    mt.ins(b);

    /* funny trick for performance */
    let bb = mem::replace(b, Box::new(new_omi_kind()));

    Ok(*bb)
}

/* omicron */
pub fn omi<'a>(inp: &'a mut Kind) -> Result<Kind, &'a mut Kind> { Err(inp) }

pub fn new_eta_kind() -> Kind { Kind::Zta {sid:None, hid:1, fnc:eta} }
pub fn new_omi_kind() -> Kind { Kind::Zta {sid:None, hid:0, fnc:omi} }

/******************************************************************/
/* remember to always give back what you dont use */

pub fn lore(end: Kind) -> Kind {
    Kind::from((
        Kind::from((new_omi_kind(), new_omi_kind())),
        Kind::from((new_eta_kind(), end))
    ))
}

pub fn lore_end() -> Kind {
    Kind::from((
        new_eta_kind(),
        Kind::from((
            Kind::from((new_omi_kind(), new_omi_kind())),
            new_eta_kind()
        ))
    ))
}

/******************************************************************/
/* remember to always give back what you dont use */

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
