fn main() {
    // PART 1 变量
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // PART 2 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is: {THREE_HOURS_IN_SECONDS}");

    // PART 3 遮蔽 覆盖
    // 遮蔽创建了一个新变量，可以改变值的类型，并且复用这个名字
    let y: u32 = 5;
    println!("first, y={y}"); //5 
    let y: u32 = y + 1;
    println!("second, y={y}"); // 6
    {
        let y: u32 = y * 2;
        println!("now, inner scope y={y}"); // 12
    }
    println!("finally,y={y}"); // 6

    let space: &'static str = "   ";
    let space: usize = space.len();
    println!("now, space is {space}");

    // 报错
    // let mut space: &'static str = "    ";
    // space = space.len();
    // println!("now ,is error")
}
