mod path_module;
// 绝对路径: 从 crate 根开始
use crate::path_module::SroceRoot;
// sroce 不是公有的，不可访问SroceOther
// use crate::path_module::other::SroceOther;
use crate::path_module::other::SroceEnum;

fn main() {
    path_module::public_fn();

    let mate: SroceRoot = SroceRoot {}; // id: 23, sroce: 78 
    println!("mate={:?}", mate);

    // sroce 不是公有的，不可访问SroceOther 报错
    // let mate_other = SroceOther {};
    // println!("mate_other={:?}", mate_other);
    // 枚举变体默认公有的
    let level1 = SroceEnum::WellDown;
    let level2 = SroceEnum::Failed;
    let level3 = SroceEnum::Great;
    println!("{:?},{:?},{:?}", level1, level2, level3);
}
