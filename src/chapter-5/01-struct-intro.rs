#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    mail: String,
    cid: u32,
}

struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

#[derive(Debug)]
struct AlwaysUnit;

fn main() {
    // PART 1 创建struct实例
    let user1 = User {
        active: false,
        name: String::from("frank"),
        mail: String::from("whatareyoutalkingabout@xyz.com"),
        cid: 321,
    };

    println!("user1.active={}", user1.active);
    println!("user1.name={}", user1.name);
    println!("user1.mail={}", user1.mail);
    println!("user1.cid={}", user1.cid);

    // PART 2 用函数创建struct实例，并利用字段初始化简写语法
    let user2 = build_user(2, String::from("CJAV"), String::from("ACA"));
    println!("user2={:?}", user2);

    // PART 3 使用结构体更新语法从其他实例创建实例， 其他实例需要放置最后
    let user3 = User {
        active: user2.active,
        name: user2.name.clone(),
        mail: user2.mail.clone(),
        cid: 3,
    };
    println!("{}=={}", user3.name, user2.name);

    let user4 = User { cid: 4, ..user1 };
    // user1 移动给了user4, 被销毁
    println!("{}", user4.name);

    // PART 4 元组结构体
    let color1 = Color(244, 244, 244);
    println!("{},{},{}", color1.0, color1.1, color1.2);
    let point1 = Point(121.00, 1212.13, 131.342);
    println!("{},{},{}", point1.0, point1.1, point1.2);

    // PART 5 类单元结构体 用于标识特定的行为或作为类型标签
    let unit1: AlwaysUnit = AlwaysUnit;
    println!("{:?}", unit1);
}

// 字段初始化简写语法
fn build_user(cid: u32, mail: String, name: String) -> User {
    User {
        active: false,
        name,
        mail,
        cid,
    }
}
