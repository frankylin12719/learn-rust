// 公有,可访问
pub fn public_fn_other() {
    println!("other Hello, world!!! Catelog module");
    catelog_inner_module::public_fn_other();
    privacy_fn_other();
}

// 默认私有（仅当前模块可见)
fn privacy_fn_other() {
    println!("other  It is not used outside, but inside!!!! ");
}

// 公有,可访问
pub mod catelog_inner_module {
    pub fn public_fn_other() {
        println!(
            "other Hello, world!!! module in catelog module!! it can use inside and outside!!"
        );
    }
}
