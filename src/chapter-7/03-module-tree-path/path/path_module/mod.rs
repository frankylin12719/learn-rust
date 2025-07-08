pub mod other;

// 公有,可访问
pub fn public_fn() {
    println!("Hello, world!!!");
    path_inner_module::public_fn();
    privacy_fn();
    // 相对路径 self::表示当前模块
    self::other::public_fn_other();
}

// 默认私有（仅当前模块可见)
fn privacy_fn() {
    println!("It is not used outside, but inside!!!!");
}

#[derive(Debug)]
pub struct SroceRoot {
    // id: i32,
    // sroce: i32,
}

// 声明公开模块（可被外部访问) 本模块的子模块
pub mod path_inner_module {
    pub fn public_fn() {
        println!("Hello, world!!! module in outside module!! it can use inside and outside!!");
    }
}
