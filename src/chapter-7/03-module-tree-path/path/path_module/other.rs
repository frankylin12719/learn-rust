// 公有,可访问
pub fn public_fn_other() {
    println!("other Hello, world!!! Catelog module");
    catelog_inner_module::public_fn_other();
    privacy_fn_other();
    // 相对路径 super:: 指向父模块
    let mate_root_in_other = super::SroceRoot {};
    println!("mate_root_in_other={:?}", mate_root_in_other);

    let mate_other_in_mod = self::catelog_for_struct(32);

    println!("mate_other_in_mod={:?}", mate_other_in_mod);
}

// 默认私有（仅当前模块可见)
fn privacy_fn_other() {
    println!("other  It is not used outside, but inside!!!! ");
}

// 公有,可访问
pub mod catelog_inner_module {
    // this function can be used only when add pub at front
    //  if not so, user only can reference catelog_inner_module but can not reference public_fn_other
    pub fn public_fn_other() {
        println!(
            "other Hello, world!!! module in catelog module!! it can use inside and outside!!"
        );
    }
}

pub fn catelog_for_struct(id: i32) -> SroceOther {
    SroceOther { id: id, sroce: 43 }
}

#[derive(Debug)]
pub struct SroceOther {
    // 该结构体只有 id 是(公有的)暴露在外的， sroce
    pub id: i32,
    sroce: i32,
}

#[derive(Debug)]
#[warn(unused_variables)]
pub enum SroceEnum {
    WellDown,
    Failed,
    Great,
}
