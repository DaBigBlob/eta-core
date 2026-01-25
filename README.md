# Eta Core (no_std)
Rust implementation of my structural calculus.

## Basic Usage
```rs
use eta_core::basic; /* import */

let mut out = String::new(); /* create output string for reuse */
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
let inp = match parse(input, &mut dict) {
    Ok(k) => k,
    Err(e) => {
        /* error implements Display so you can print it (or match on ParserErr) */
        eprintln!("P[!]: {}\n", e);
        return;
    }
};

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
((E ((A A) E)) ;main
;Lx
(E (
(P (E
;Px
;lambda calculus expression here
;note: P is application, A is abstraction as we have set up here
(P ((A (x (x x))) x))
;/lambda
;/Px
))
;Pi
(
((A (n b)) x)
(E ((n b) x))
)
;/Pi
))
;/Lx
) ;/main
```

> Documentation under heavy WIP.
