fn main() {
    // PART 1 函数顺序执行
    println!("HELLO WORLD!!");
    print_new();

    // PART 2 带参数函数
    print_with_parma(66);
    print_with_parmas(66, 66.66, 'a');

    // PART 3 语句和表单式
    expression_fn();
    let return_val = has_return();
    println!(" the return value is {return_val}");

    // 不报错 表达式会返回值
    let return_add_val1 = has_return_add1(5);
    println!("1 the return value is {return_add_val1}");

    // 报错 语句不会返回值
    // let return_add_val2 = has_return_add2(5);
    // println!("2 the return value is {return_add_val2}");

    let return_add_val3 = has_return_add3(5);
    println!("3 the return value is {return_add_val3}");

}

fn print_new() {
    println!("HELLO AGAIN!!");
}

fn print_with_parma(p: i32) {
    println!("The value of parmameter is: {p}");
}

fn print_with_parmas(p1: i32, p2: f64, p3: char) {
    println!("The value of 1st parmameter is: {p1}");
    println!("The value of 2nd parmameter is: {p2}");
    println!("The value of 3rd parmameter is: {p3}");
}

fn expression_fn() {
    // 报错
    // let x = (let y = 5 );
    //  不报错
    let x = 
    // 表达式 代码块
    {
        let y = 5; // 语句
        y + 1 
    };

    println!("x is {x}")
}

fn has_return() -> i32{
    5
}

fn has_return_add1(x:i32) -> i32{
    x+1
}

// 报错 语句不会返回值
// fn has_return_add2(x:i32) -> i32{
//     x+1;
// }

// 显式返回 有返回值 
fn has_return_add3(x:i32) -> i32{
    return x+1;
}
