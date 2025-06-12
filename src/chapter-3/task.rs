use std::io;

fn main() {
    // 温度转换
    temp_c_to_f();

    // 生成第n个斐波那契数
    gen_fs(12);

    // 打印歌词
    print_christmas();
}

fn temp_c_to_f() {
    let mut temp_c: String = String::new();
    io::stdin()
        .read_line(&mut temp_c)
        .expect("fail to read line");

    let temp_c: f64 = temp_c.trim().parse().expect("it should be a number");

    // 保留3位小数
    let temp_f: f64 = (temp_c * 1.8 + 32.0 * 1000.0).round() / 1000.0;

    println!("temp_h is {}", temp_f);

    // 保留3位小数
    // println!("temp_h is {:.3}", temp_f);
}

fn gen_fs(num: i32) {
    let mut num0 = 0;
    let mut num1 = 1;
    let mut fs = 0;
    let mut cnt = 0;
    while cnt < num {
        if num - 1 == 1 {
            fs = num1;
            break;
        }
        if num - 1 == 0 {
            fs = num0;
            break;
        }
        num0 = num1;
        num1 = fs;
        cnt += 1;
        fs = num1 + num0;
    }

    println!("Number {num} is {fs}");
}

fn print_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let mut cnt = 0;
    for ele in days {
        let mut gg: String = String::new();
        // (N..M) <=> [N,N+1,N+2,...,M-1]
        for g in (0..cnt + 1).rev() {
            if g == 0 {
                if cnt == 0 {
                    gg = gg + gifts[g];
                } else {
                    gg = gg + " And " + gifts[g];
                }
            } else {
                gg = gg + gifts[g] + ","
            }
        }
        println!(
            "On the {} day of Christmas, my true love sent to me: {}",
            ele, gg
        );
        cnt += 1;
    }
}
