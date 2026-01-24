
use crate::{human::*, theory::*};

use alloc::string::String;
use core::{fmt::Write, str};

/* the thing that does */
pub fn runner(out: &mut String, input: &str) {
    let mut dict = Dict::new();

    let inp = match parse(input, &mut dict) {
        Ok(k) => k,
        Err(e) => {
            let _ = write!(out, "P[!]: parse error at byte {}: {}\n", e.pos, e.msg);
            return;
        }
    };

    let mut exp = lore(Kind::from((inp, lore_end())));
    let _ = write!(out, "I: {}\n", unparse(&exp, &dict));

    match eta(&mut exp) {
        Ok(res) => { let _ = write!(out, "E[^]: {}\n", &unparse(&res, &dict)); }
        Err(res) => { let _ = write!(out, "E[H]: {}\n", &unparse(&res, &dict)); }
    }
}
