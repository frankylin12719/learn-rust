fn main() {
    // PART 1 不用slice找出字符串第一个单词
    let str = String::from("iviuqi ucbiuquq cbqiu ucbqui svqv");
    let res = task_find_first_word(&str);
    // res 和 str 不一定同步
    // str.clear(); // 执行这一步，取不到word,但是 res 仍对应的值
    // PART 2 字符串slice
    let word = &str[0..res];
    println!("res={res}, str={str}, word={word}");

    // PART 3 用slice找出字符串第一个单词
    let slice_word = slice_first_word(&str);
    // slice_word 和 str 同步
    // str.clear(); // 执行这一步，slice_word也取不到
    println!("str={str}, slice_word={slice_word}");

    // PART 4 字符串字面值就是 slice
    let slice_str1: &str = "HELLO WORLD"; // 一个不可变引用
    let slice_str2 = String::from("HELLO WORLD");
    let ss2 = &slice_str2;
    let judge = ss2 == &slice_str1;
    println!("{judge}");
}

fn task_find_first_word(str: &String) -> usize {
    let bytes = str.as_bytes(); // 创建字节数组

    // (i, &item) 模式结构——第一个元素是索引，第二个元素是集合中元素的引用（单字节）
    // iter 返回集合的每个元素，enumerate包装的iter返回的结果
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 返回值
            return i;
        }
    }
    // 没有空格的返回值
    str.len()
}

fn slice_first_word(str: &str) -> &str {
    let bytes = str.as_bytes(); // 创建字节数组

    // (i, &item) 模式结构——第一个元素是索引，第二个元素是集合中元素的引用（单字节）
    // iter 返回集合的每个元素，enumerate包装的iter返回的结果
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 返回值
            return &str[0..i];
        }
    }
    // 没有空格的返回值
    &str[..]
}
