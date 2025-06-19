mod catelog_module;

// src/
// └── main.rs
// └── catelog_module
//     └── mod.rs 根模块，命名一定是mod.rs
//     └── other.rs
fn main() {
    catelog_module::public_fn_root();
    // outside_module::privacy_fn(); // 报错
    catelog_module::other::public_fn_other();
}
