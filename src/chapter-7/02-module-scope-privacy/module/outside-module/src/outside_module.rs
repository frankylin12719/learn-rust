// 公有,可访问
pub fn public_fn() {
    println!("Hello, world!!!");
    outside_inner_module::public_fn();
    privacy_fn();
}

// 默认私有（仅当前模块可见)
fn privacy_fn() {
    println!("It is not used outside, but inside!!!!");
}

// 声明公开模块（可被外部访问)
pub mod outside_inner_module {
    pub fn public_fn() {
        println!("Hello, world!!! module in outside module!! it can use inside and outside!!");
    }
}
