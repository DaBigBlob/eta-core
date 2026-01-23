# Eta Combinator Core (no_std)

## Basic Usage
```rs
use eta_core::basic::runner; /* import */

let mut out = String::new(); /* create output string for reuse */
runner(&mut out, &input); /* run the runner */
print!("{out}"); /* use it however */
```

## Advanced Usage
```rs
use crate::{human::*, theory::*}; /* import */

/* create new human readable name dictionary */
let mut dict = Dict::new();

/* run the parser */
let inp = match parse(input, &mut dict) {
    Ok(k) => k,
    Err(e) => {
        let _ = write!(out, "parse error at byte {}: {}\n", e.byte, e.msg);
        return;
    }
};

/* default/baisc lore */
/* (you may create your own lore using eta_core::theory) */
/* you may also create bring your zeta extentions! (using Kind::try_from((zeta_fn, ID))) */
let mut exp = lore(Kind::from((inp, lore_end())));

match eta(&mut exp) {
    Ok(res) => { let _ = write!(out, "[^] {}\n", unparse(&res, &dict)); } /* (eta could not be consumed) */
    Err(res) => { let _ = write!(out, "[H] {}\n", unparse(&res, &dict)); } /* halt (eta is consumed) */
}
```

> Documentation under heavy WIP.
