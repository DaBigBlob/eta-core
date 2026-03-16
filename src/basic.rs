use crate::{
    human::{Dict, Parser, View},
    theory::{eta, lore, lore_end, Kind},
};
use core::fmt::Write;

/* does the basic default */
pub fn execute<O: Write, I: Iterator<Item = char>>(out: &mut O, input: I) {
    let mut dict = Dict::new();

    let mut prs = Parser::new(input);
    let inp = match prs.parse_spair(&mut dict) {
        Ok(k) => k,
        Err(err) => {
            let _ = write!(out, "P[!]: {}\r\n", err);
            return;
        }
    };

    /* check for garbage at the end */
    if let Some(at) = prs.has_more() {
        let _ = write!(out, "X[!]: garbage after {at} chars\r\n");
        return;
    }

    let mut exp = lore(Kind::from((inp, lore_end())));
    let _ = write!(out, "I: {}\r\n", View::new(&exp, &dict));

    match eta(&mut exp) {
        Ok(res) => {
            let _ = write!(out, "E[^]: {}\r\n", View::new(&res, &dict));
        }
        Err(res) => {
            let _ = write!(out, "E[H]: {}\r\n", View::new(&res, &dict));
        }
    }
}
