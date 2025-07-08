fn main() {
    // part1 创建Vector
    let v1: Vec<i32> = Vec::new();
    println!("v1={:?}", v1);
    // 不限长的单一数据类型数组

    let v2 = vec![1, 2, 3, 4, 5, 6];
    println!("v2={:?}", v2);
    println!("v2[1]={}", v2[1]);

    // part2 更新Vector
    let mut v3: Vec<i32> = Vec::new();
    // 从末尾开始添加
    v3.push(3);
    v3.push(4);
    v3.push(5);
    // 从末尾开始删除，并返回删除的值
    v3.pop();
    println!("v3={:?}", v3);

    // part3 读取元素
    let v4 = vec![23, 121, 13123, 4341, 12];
    let sec1 = &v4[2];
    println!("sec1 = {}", sec1);
    println!("sec1= v4[2] {}", *sec1 == v4[2]);

    let sec2 = v4.get(2);
    println!("sec2 = {:?}", sec2);
    match sec2 {
        Some(val) => println!("there is {}", val),
        None => println!("there is no value"),
    }

    // 报错
    // let none_exist = &v4[10];
    // println!("{:?}", none_exist);

    let none_exist = v4.get(10);
    println!("none_exist = {:?}", none_exist);
    match none_exist {
        Some(val) => println!("there is {}", val),
        None => println!("there is no value"),
    }

    let mut v5 = vec![12, 11, 14424, 25135, 252];
    let first = &v5[0];
    //未报错
    println!("first={:?}", first);
    // // 报错
    // // 不能在相同作用域中同时存在可变和不可变引用的规则
    // v5.push(131);
    // // 在 vector 的结尾增加新元素时，
    // // 在没有足够空间将所有元素依次相邻存放的情况下，
    // // 可能会要求分配新内存并将老的元素拷贝到新的空间中
    // // 索引值引用会指向被释放的内存中,不符合所有权规则
    // println!("first={:?}", first);

    // part4 遍历元素
    let v6 = vec![52, 2525, 45646, 251, 686];
    for i in &v6 {
        println!("i={}", i);
    }
    println!("v6={:?}", v6);

    let mut v7 = vec![52, 2525, 45646, 251, 686];
    for i in &mut v7 {
        *i += 20;
        println!("i={}", i);
    } //可变引用结束
    println!("v7={:?}", v7);

    // part5   枚举类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v8 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("v8={:?}", v8);
    println!("&v8[0]={:?}", &v8[0]);

    {
        let v9 = vec![134, 1341, 3653, 25251];
    } //v9被释放， 其中的所有元素也丢弃释放
    // 报错
    // println!("V9={:?}", v9);
}
