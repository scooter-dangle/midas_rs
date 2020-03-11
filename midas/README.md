midas_rs
========

Rust implementation of
[https://github.com/bhatiasiddharth/MIDAS](https://github.com/bhatiasiddharth/MIDAS)

```toml
# Cargo.toml
[dependencies]
midas_rs = "0.2"
```

```rust
use midas_rs::{Int, Float, MidasR};

fn main() {
    // For configuration options, refer to MidasRParams
    let mut midas = MidasR::new(Default::default());

    println!("{:.6}", midas.insert((1, 1, 1)));
    println!("{:.6}", midas.insert((1, 2, 1)));
    println!("{:.6}", midas.insert((1, 1, 2)));
    println!("{:.6}", midas.insert((1, 2, 3)));

    assert_eq!(midas.insert((1, 2, 4)), midas.query(1, 2));
}
```
