// // 用Result枚举类处理错误
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    // part 1
    // let get_file_result = File::open("Hello.txt");

    // let _get_file = match get_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("oooooh!!!it happend!{error:?}"),
    // };

    // part 2
    // let get_file_result2 = File::open("Hello.txt");

    // let _get_file2 = match get_file_result2 {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("Hello.txt") {
    //             // 不存在就创建
    //             Ok(fc) => fc,
    //             Err(e) => panic!("it does not exist and can not create!!{e:?}"),
    //         },
    //         _ => {
    //             panic!("it exists but can not open!!:{error:?}")
    //         }
    //     },
    // };

    // part 3
    // unwrap_or_else 专用于Option<T>和Result 解开闭包然后遇错判断
    // let get_file_result3 = File::open("Hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("Hello.txt").unwrap_or_else(|error| {
    //             panic!("it does not exist and can not create!!{error:?}");
    //         })
    //     } else {
    //         panic!("it exists but can not open!!:{error:?}");
    //     }
    // });

    // part4
    // unwrap() expect() panic的快捷方式
    // let get_file_result4 = File::open("Hello.txt").unwrap();
    // let get_file_result4_1 = File::open("Hello.txt").expect("panic!!");

    let _ = read_from_file();
}

// 传播错误
fn read_from_file() -> Result<String, io::Error> {
    let get_file_result5 = File::open("Hello.txt");

    let mut file_text = match get_file_result5 {
        Ok(file) => file,
        Err(e) => {
            println!("file_text");
            return Err(e);
        }
    };

    let mut file_str = String::new();

    match file_text.read_to_string(&mut file_str) {
        Ok(_) => {
            println!("{}", file_str);
            return Ok(file_str);
        }
        Err(e) => {
            println!("file_str");
            return Err(e);
        }
    }
}

// ?运算符 传播错误的快捷方式
// read_from_file2()与read_from_file()等效
fn read_from_file2() -> Result<String, io::Error> {
    let mut get_result_file6 = File::open("Hello.txt")?;
    let mut file_str = String::new();
    get_result_file6.read_to_string(&mut file_str)?;
    // File::open("Hello.txt")?.read_to_string(&mut file_str)?;
    Ok(file_str)
}
