use std::fmt::format;

#[derive(Debug)]
enum QuarterType {
    Dollar,
    Pound,
}

impl QuarterType {
    fn existed_in(&self, radio: f32) -> bool {
        match self {
            QuarterType::Dollar => radio < 0.75,
            QuarterType::Pound => radio < 1.00,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(QuarterType),
}

fn main() {
    // PART 1 if let
    let config_max = Some(45);
    match config_max {
        Some(max) => println!("the max is {max}"),
        _ => (),
        // _ => println!("it is none"),
    }

    // 等效为
    // if let 语法获取通过等号分隔的一个模式和一个表达式
    if let Some(max) = config_max {
        println!("the max is {max}");
    }
    //     else {
    //         println!("it is none");
    //     }

    // PART 2 let ... else
    let res = quarter_type_case(Coin::Quarter(QuarterType::Dollar));
    println!("res={res:?}");
}

fn quarter_type_case(coin: Coin) -> Option<String> {
    // ep 1
    // if let Coin::Quarter(country) = coin {
    //     if country.existed_in(0.88) {
    //         println!("ep 1 {country:?} is pound");
    //         Some(format!("ep 1 {country:?} is pound"))
    //     } else {
    //         println!("ep 1 {country:?} is dollar");
    //         Some(format!("ep 1 {country:?} is dollar"))
    //     }
    // } else {
    //     None
    // }

    // ep 2
    // let country = if let Coin::Quarter(country) = coin {
    //     country
    // } else {
    //     return None;
    // };

    // if country.existed_in(0.88) {
    //     println!("ep 2 {country:?} is pound");
    //     Some(format!("ep 2 {country:?} is pound"))
    // } else {
    //     println!("ep 2 {country:?} is dollar");
    //     Some(format!("ep 2 {country:?} is dollar"))
    // }

    // eq 3
    let Coin::Quarter(country) = coin else {
        return None;
    };

    if country.existed_in(0.88) {
        println!("ep 3 {country:?} is pound");
        Some(format!("ep 3 {country:?} is pound"))
    } else {
        println!("ep 3 {country:?} is dollar");
        Some(format!("ep 3 {country:?} is dollar"))
    }
}
