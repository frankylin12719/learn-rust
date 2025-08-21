use std::collections::HashMap;

fn main() {
    // Part 1 创建&索引&遍历
    let mut map1 = HashMap::new();

    // 所有的键必须是相同类型，值也必须都是相同类型。
    map1.insert("a", 10);
    map1.insert("b", 1);

    let key1 = "a";
    let value1 = map1.get(key1).copied().unwrap_or(0);
    println!("value1={value1}");
    let key_none = "c";
    let value_none = map1.get(key_none).copied().unwrap_or(0);
    // .copied().unwrap_or(0); 在对应map中没有该键时获取一个Option<i32>并置为0
    println!("value_none={value_none}");

    // 任意顺序打印出每一个键值对
    for (key, value) in &map1 {
        println!("{key}: {value}");
    }

    // Part 2 所有权

    let key_str = String::from("key_string");
    let value_str = String::from("value_string");

    let mut map_str = HashMap::new();

    map_str.insert(key_str, value_str); //  --------- value moved here

    println!("{map_str:?}");

    // println!("key_str={key_str}"); //报错 value borrowed here after move 所有权已经转义只map_str
    // println!("value_str={value_str}");
    let key_str_get = String::from("key_string");
    let value_str_get = map_str.get(&key_str_get);
    println!("value_str_get={:?}", value_str_get);

    for (key, value) in &map_str {
        println!("{key}: {value}");
    }

    // Part 3 覆盖更新
    let key_str_new = String::from("key_string");
    let value_str_new = String::from("new_value_string");

    map_str.insert(key_str_new, value_str_new);

    for (key, value) in &map_str {
        println!("{key}: {value}");
    }

    map_str
        .entry(String::from("key_string"))
        .or_insert(String::from("empty"));
    // entry检查键参数是否赋值
    map_str
        .entry(String::from("key_string_new_empty"))
        .or_insert(String::from("empty"));

    println!("{map_str:?}");

    println!("{:?}", map_str.entry(String::from("key_string")));

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 引用
        let count = map.entry(word).or_insert(0);
        print!("count = {} ", count); //count = 0 count = 0 count = 0 count = 1
        // 解引用赋值
        *count += 1;
    }

    println!("{map:?}"); // {"hello": 1, "world": 2, "wonderful": 1
}
