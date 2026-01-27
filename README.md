# Eta Core (no_std)
Rust implementation of my structural calculus.

## Install
```bash
cargo add eta-core
```

## Basic Usage
```rs
use eta_core::basic; /* import */

let mut out = String::new(); /* create output string for reuse */
/* assuming "input" is &str or String */
basic::execute(&mut out, input.chars().into_iter()); /* run the executor */
print!("{out}"); /* use it however */
```

## Advanced Usage
```rs
/* this is essentially the implementation of eta_core::basic::runner */

use eta_core::{human::*, theory::*}; /* import */

/* create new human readable name dictionary */
let mut dict = Dict::new();

/* run the parser */
/* "input" must implement Iterator<Item = char> */
/* for &str and String, you may .chars().into_iter() */
let mut prs = Parser::new(input);
let inp = match prs.parse_spair(&mut dict) {
    Ok(k) => k,
    Err(err) => {
        /* error implements Display so you can print it (or match on ParserErr) */
        eprintln!(out, "P[!]: {}\n", err);
        return;
    }
};

/* check for garbage at the end */
if let Some(at) = prs.has_more() {
    eprintln!(out, "X[!]: garbage after {at} chars\n");
    return;
}

/* default/baisc lore */
/* (you may create your own lore using eta_core::theory) */
/* you may also create bring your zeta extentions! (using Kind::try_from((zeta_fn, ID))) */
let mut exp = lore(Kind::from((inp, lore_end())));

match eta(&mut exp) {
    Ok(res) => println!("E[^]: {}\n", View::new(&res, &dict)), /* (eta could not be consumed) */
    Err(res) => println!("E[H]: {}\n", View::new(&res, &dict))  /* halt (eta is consumed) */
}
```

## Lambda Calculus
Lambda calculus can be implemented as a sub-calculus of Eta.

```lisp
;a comment starts with ; and continues to end of line
((E ((A A) E)) ;external capture of eta and omicron

(E (; invoking eta
(P ;capturing lambda application implementation from below
;<---lambda expression begin--->

;NOTE: as we have described,
;   application is of form (E (P (<lambda abstraction>  <applicand>)))
;   abstraction is of form (A (<binding variabel> <body>))
;NOTE: alpha renaming is not a real thing, make sure to name accordingly

;example 1: omega combinator
;uncomment the following to make your stack overflow ☺️
; (E (P (
; (A (y (E (P (y y)))))
; (A (y (E (P (y y)))))
; )))

;example 2: make 4 of the virus 🦠
(E(P ((A (v ((v v) (v v)))) 🦠)))

;<---lambda expression end--->
)

(;this is the lambda application implementation to be used as P
((A (n b)) x)
(E ((n b) x))
)

)))
```

> Documentation under heavy WIP.
