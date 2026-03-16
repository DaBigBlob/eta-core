# Eta Core (no_std)
Freestanding Rust implementation of my structural calculus.

## In persuit of answers to
- How small can a language be?
- Can a language be so small that the only primitive of that language is the lanaguage interpreter itself?
- Can we create a keyword-less programming language?
- Can we create a turing complete calculus of structural morphisms - in particular, as an automaton on tree structure?
- Can we represent Types (and stratified Type universes) in terms of structures (with holes)?
- What if we could rationalize interpretation of a program as the result a temporal tower of self-evaluation with *the latest evaluation* being that of the impurity (not self-eval) program?

## Syntax/Semantics
- **`S-Pair`**: Every program is a `S-Pair` i.e. an `S-expression` with exactly two members. This just the default surface syntax (`src/human.rs`) and is completely separate from the actual (AST) evaluator (`src/theory.rs`), and hence cna be pretty trivially changed.
- **Keywords**: There are no keywords in this programming language!
- **Metacircularity**: The Eta evaluator is also just a program of the Eta language.
- **Fundamental Axioms/primitives**: The Eta language has 2 axioms (i.e. inhabitants of the lowest Type universe):
    - The (existence of) evaluator itself (called Eta) (created using `new_eta_kind()`).
    - The trivial O(1) computation/evaluation (called Omicron) (created using  `new_omi_kind()`).
- **Extensibility**: More axioms can be trivially added (using `Kind::TryFrom(newAxiom, axiomIdentity)`). The metacircular temporal tower (default from `src/theory.rs`) can be replaced with a custom one.

## Install
```bash
cargo add eta-core
```
## Usage
### Basic Usage
```rs
use eta_core::basic; /* import */

let mut out = String::new(); /* create output string for reuse */
/* assuming "input" is &str or String */
basic::execute(&mut out, input.chars().into_iter()); /* run the executor */
print!("{out}"); /* use it however */
```
ℹ️ **Note**: `basic::execute` returns a `I:` part (input) and a `E:` part (evaluated), separated by newline. The `I:` is the Eta expression is what the parser understands from the actual input. The `E:` part is what is returned from the evaluator.

### Advanced Usage
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
Lambda calculus can be implemented as a sub-calculus of Eta. (Evidence for turing completeness.)

```lisp
;a comment starts with ; and continues to end of line
((E ((A A) E)) ;external capture of eta and omicron

(E (; invoking eta
(P ;capturing lambda application implementation from below
;<---lambda expression begin--->

;NOTE: as we have described,
;   application is of form (E (P (<lambda abstraction>  <applicand>)))
;   abstraction is of form (A (<binding variabel> <body>))
;NOTE: eta does not alpha rename automatically, make sure to name accordingly

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

## Generative-AI/LLM
This is and will be a **generative-AI/LLM free** codebase. (Makes no sense to outsource human curiocity to computers lol)

## ⚠️ **TODO**
- Complete motivation: power, simplicity, etc
- Complete formal documentaion: small-step inference rules and BNF
- Complete guide to programming in this language
