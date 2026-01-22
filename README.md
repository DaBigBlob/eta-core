# Eta Combinator Core (no_std)

## Basic Usage
```rs
use eta_core::runner::runner; /* import */

/* use */
pub fn run(input: &str) -> String {
    let mut out = String::new();
    runner(&mut out, input);
    out
}
```

> Documentation under heavy WIP.
