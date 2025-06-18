#[derive(Debug)]
enum QuarterType {
    Dollar,
    Pound,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(QuarterType),
}

fn main() {
    // PART 1 match控制流
    let penny = value_cents(Coin::Penny);
    println!("penny is {}", penny);
    let dime = value_cents(Coin::Dime);
    println!("dime is {}", dime);
    let quarter = value_cents(Coin::Quarter(QuarterType::Dollar));
    println!("quarter is {}", quarter);

    // PART 2 匹配Option<T>
    let init_val = Some(1);
    let add = plus_one(init_val);
    let none = plus_one(None);
    println!("init_val = {init_val:?}");
    println!("add = {add:?}");
    println!("none = {none:?}");

    // PART 3 通配模式 和 _占位符
    let roll = 3;
    match roll {
        1 => println!("YOU LOSE!!"),
        6 => println!("YOU WIN!!"),
        // other => move_forward(other),
        // _ => (), // nothing happen
        _ => roll_again(),
    }
}

fn value_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("call penny");
            1
        }
        Coin::Dime => 5,
        Coin::Nickel => 10,
        Coin::Quarter(country) => {
            println!("country is {country:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 注释掉这一行会报错
        Some(i) => Some(i + 1),
    }
}

fn move_forward(roll: i32) {
    if roll < 4 {
        println!("delete one!!");
    }
    if roll > 3 {
        println!("add one!!");
    }
}

fn roll_again() {
    println!("please roll again!!");
}
