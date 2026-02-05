fn main() {
    // let r;
    // {
    //     //x 的作用域（生命周期）小于r
    //     let x = 5;
    //     r = &x;
    //     //r引用x 之后但是x内存被清空
    // }
    // //r 引用x 之后但是x内存被清空 造成悬垂引用
    // println!("2 r={r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// fn longest(x: &str, y: &str) -> &str { // expected named lifetime parameter
//     if x.len() > y.len() { x } else { y } //并不知道将要返回的引用是指向 x 或 y
// }

// 指定了签名中所有的引用必须有相同的生命周期 'a
// 实际意义是返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致
// 返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效
// 返回的引用值的生命周期要与参数有关联，参数要显式的指定生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


// 结构体亦是如此
struct ImportantExcerpt<'a> {
    part: &'a str,
}