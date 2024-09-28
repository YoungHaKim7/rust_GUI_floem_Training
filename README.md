# rust_GUI_floem_Training
https://github.com/lapce/floem

<div align="center">

# Floem

A native Rust UI library with fine-grained reactivity

[![crates.io](https://img.shields.io/crates/v/floem.svg)](https://crates.io/crates/floem)
[![docs.rs](https://docs.rs/floem/badge.svg)](https://docs.rs/floem)
[![Discord](https://img.shields.io/discord/946858761413328946?color=%237289DA&label=discord)](https://discord.gg/RB6cRYerXX)

</div>

_The project is still maturing. We will make occasional breaking changes and add missing features on our way to v1._

## Quickstart


```rust
use floem::{
    reactive::create_signal,
    views::{button, label, Decorators},
    IntoView,
};

fn app_view() -> impl IntoView {
    // Create a reactive signal with a counter value, defaulting to 0
    let (counter, mut set_counter) = create_signal(0);

    // Create a vertical layout
    (
        // The counter value updates automatically, thanks to reactivity
        label(move || format!("Value: {counter}")),
        // Create a horizontal layout
        (
            button("Increment").action(move || set_counter += 1),
            button("Decrement").action(move || set_counter -= 1),
        ),
    ).style(|s| s.flex_col())
}

fn main() {
    floem::launch(app_view);
}
```

