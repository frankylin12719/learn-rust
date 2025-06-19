mod inner_module {
    // pp 公有
    pub fn public_fn() {
        println!("Hello, world!!!");
        privacy_fn();
    }

    // 默认私有（仅当前模块可见)
    fn privacy_fn() {
        println!("It is not used outside, but inside!!!!");
    }
}

// src/
// └── main.rs
fn main() {
    inner_module::public_fn();
    // inner_module::privacy_fn(); // 报错
}
