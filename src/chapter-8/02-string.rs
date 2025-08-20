fn main() {
    // Part 1 字符串借用 &str
    let mut s1 = String::new();

    let s1 = "initial contents".to_string();
    // 等效为 （字符串为UTF-8编码的）
    let s1 = String::from("initial contents");

    // Part 2 更新字符串
    let mut s2 = String::from("first");
    println!("s2={}", s2);
    s2.push_str(" second ");
    println!("s2={}", s2);
    s2.push('3');
    println!("s2={}", s2);
    s2.push('f');
    println!("s2={}", s2);

    // Part3 拼接字符串
    let s3_1 = String::from("s31,");
    let s3_2 = String::from("s32!");
    let s3_3 = s3_1 + &s3_2;
    // println!("s3_1={}", s3_1); // s3_1被移动了，已经销毁 调用了add函数, 获取了s1的所有权， &String可以强转为&str
    println!("s3_2={}", s3_2);
    println!("s3_3={}", s3_3);

    let s3_4_1 = String::from("one");
    let s3_4_2 = String::from("two");
    let s3_4_3 = String::from("three");
    let s3_4 = format!("{s3_4_1}-{s3_4_2}-{s3_4_3}"); // format宏
    let s3_4_same = s3_4_1 + "-" + &s3_4_2 + "-" + &s3_4_3;
    println!("s3_4={}", s3_4);
    println!("s3_4_same={}", s3_4_same);

    // Part 4 索引字符串
    let s4 = String::from("Hello");
    // let s4_part1 = &s4[0]; // string indices are ranges of `usize` Rust 会直接报错
    // println!("s4={s4},s4_char1={s4_part1}");
    // 字符串不支持索引
    // Rust 不允许使用索引获取 String 字符的原因是，（string类型采用utf-8编码（不定长字节编码））索引操作预期总是需要常数时间（O(1)）。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符
    let s4_char = String::from("bl索引");
    let s4_char_part1 = &s4_char[0..1];
    println!("s4_char_part1={s4_char_part1}");

    // let s4_char_part4 = &s4_char[0..4];  // 报错byte index 4 is not a char boundary; it is inside '索' (bytes 2..5) of `bl索引`
    // println!("s4_char_part4={s4_char_part4}");

    for c in s4_char.chars() {
        println!("c={c}");
    }

    for b in s4_char.bytes() {
        println!("b={b}");
    }
    let chars: Vec<char> = s4_char.chars().collect();
    println!("char.length={},byte.length={}", chars.len(), s4_char.len())
    // 可以发现 c和b不一定是一一对应的，因为rust中字符串为utf-8不定长编码
}
