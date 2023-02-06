# Precond

Toy repo to explore Rust procedural macros. Annotate functions with the `precond` attribute macro to execute a transformation on input arguments.

```Rust
use precond::precond;

#[precond(|a, b| if b == 0 { (b, a) } else { (a, b) })]
fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // -- snip --
}

#[precond(|a, b| (std::cmp::min(a, 100), std::cmp::min(b, 100)))]
fn clipped_add(a: u8, b: u8) -> u8 {
    a + b
}
```
