fn main() {
    // 调用宏
    println!("Hello, world!");
    // 调用函数
    // 注意这里调用的是函数 println，而不是宏
    println("not marco");
}

fn println(message: &str) {
    std::println!("{}", message);
}
