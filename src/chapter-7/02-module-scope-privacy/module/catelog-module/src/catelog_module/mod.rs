// 公有,可访问
pub fn public_fn_root() {
    println!("ROOT Hello, world!!! Catelog module");
    catelog_inner_module::public_fn_root();
    privacy_fn_root();
}

// 默认私有（仅当前模块可见)
fn privacy_fn_root() {
    println!("ROOT It is not used outside, but inside!!");
}

// 公有,可访问
pub mod catelog_inner_module {
    pub fn public_fn_root() {
        println!("ROOT Hello, world!!! module in catelog module!! it can use inside and outside!!");
    }
}

pub mod other;
