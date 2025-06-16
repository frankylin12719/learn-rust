fn main() {
    // PART 1 引用
    let str = String::from("Reference");

    let (len, str_new) = calc_str(str);
    // str失效移动给了str_new
    println!("the length of '{str_new}' is {len}");

    // &str_new 提供了str_new值的引用（允许使用值但不获取其所有权,当引用停止使用时，它所指向的值也不会被丢弃），就像指针地址，
    // 与指针不同的是，引用在其生命周期内保证指向某个特定类型的有效值
    let len_new = calc_len(&str_new);
    // str_new还是有效的
    println!("NEW FUNCTION: the length of '{str_new}' is {len_new}");

    // PART 2 借用：创建一个引用的行为

    // let str_bro = String::from("hello");
    // 报错，只有使用权，没有所有权（不可更改，只可查询应用）
    // change(&str_bro);

    // PART 3 可变引用
    let mut str_bro_mut = String::from("hello");
    change_mut(&mut str_bro_mut);
    println!("new str_bro_mut is {str_bro_mut}");
}

fn calc_str(s: String) -> (usize, String) {
    let l = s.len();
    (l, s)
}

fn calc_len(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

// fn change(s: &string) {
//     s.push_str(", world");
// }

fn change_mut(s: &mut String) {
    s.push_str(", world");
}
