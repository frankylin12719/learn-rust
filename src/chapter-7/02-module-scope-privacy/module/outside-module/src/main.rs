mod outside_module;

// src/
// └── main.rs
// └── outside_module.rs
fn main() {
    outside_module::public_fn();
    // outside_module::privacy_fn(); // 报错
    outside_module::outside_inner_module::public_fn();
}
