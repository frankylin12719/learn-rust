use std::io;

fn main() {
    // PART 1 静态类型
    let guess: u32 = "42".parse().expect("NOT A NUMBER!");
    println!("str to number: {guess}!");

    // PART 2 标量
    // 整型
    let a1: u8 = 21;
    let a1_1 = 21u8;
    let a1_2: u8 = 2_1;

    let same1_1: bool = a1 == a1_1;
    let same1_2: bool = a1_2 == a1_1;
    println!("a1={a1}, a1_1={a1_1}, a1_2={a1_2}"); // a1=21, a1_1=21, a1_2=21,
    println!("same1_1:{same1_1}, same1_2:{same1_2}"); // same1_1:true, same1_2:true

    let b1: i8 = -21;
    let b1_1 = -21i8;
    let b1_2: i8 = -2_1;

    let sbme1_1: bool = b1 == b1_1;
    let sbme1_2: bool = b1_2 == b1_1;
    println!("b1={b1}, b1_1={b1_1}, b1_2={b1_2}"); // b1=21, b1_1=21, b1_2=21,
    println!("sbme1_1:{sbme1_1}, sbme1_2:{sbme1_2}"); // sbme1_1:true, sbme1_2:true

    let num_10 = 98_222;
    let num_16 = 0x1212;
    let num_8 = 0o1211;
    let num_2 = 0b1010;
    let num_byte = b'A';

    println!("num_10:{num_10}, num_16:{num_16}, num_8:{num_8}, num_2:{num_2}, num_byte:{num_byte}");
    println!("{},{},{},{},{}", num_10, num_16, num_8, num_2, num_byte);

    // 整型溢出
    // let c: u8 = 256; // error: literal out of range for `u8`

    // 浮点型  默认是f64类型（双精度），f32是单精度
    let xf = 2.0;
    let xf2: f32 = 3.0;
    println!("{xf}, {xf2}");

    // 数值运算 只能同类型进行运算
    let add = 5 + 10;
    let mius = 95 - 2;
    let mult = 3 * 0x24;
    let divide1 = 323 / 113;
    let divide2 = -4 / 3;
    let rest = 43 % 2;
    println!(
        "{}, {}, {}, {}, {}, {}",
        add, mius, mult, divide1, divide2, rest
    );

    // 布尔类型 bool
    let t = 2 == 2;
    let f = false;
    println!("{}, {}", t, f);

    // 字符类型 char

    let str1: char = 'c';
    let str2: &'static str = "aaa";
    println!("{}, {}", str1, str2);

    // PART 2 复合类型
    // 元组 每个索引的类型不一致，声明时固定长度
    let tup: (i32, char, f64) = (21, '2', 23.4422);
    // 模式匹配，解构赋值
    let (i1, i2, i3) = tup;
    println!("i1={}, i2={}, i3={}", i1, i2, i3);
    let one = tup.0;
    let two = tup.1;
    let three = tup.2;
    println!("one={}, two={}, three={}", one, two, three);
    // println!("{}", tup);

    //数组
    let arr: [i32; 5] = [23, 313, 13, 131, 13];
    let [a1, a2, a3, a4, a5] = arr;
    let arr1: i32 = arr[1];
    println!("{a1}, {a2}, {a3}, {a4}, {a5}");
    println!("{arr1},{a2}");

    let arr_same: [i32; 6] = [4; 6];
    let six: i32 = arr_same[1];
    println!("arr_same[1]={six}");

    // 无效数组访问
    let vaild_arr: [i32; 6] = [1, 2, 3, 4, 5, 6];

    let mut idx: String = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("Falied to read line");

    let idx: usize = idx.trim().parse().expect("Index entered waa not a number");

    let ele = vaild_arr[idx];
    // idx > 6 之后会触发panic 导致代码错误而退出运行 ———— 安全原则！！！
    println!("this value in {idx} is {ele}!!");
}
