// // 用Result枚举类处理错误
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use core::error;
use std::{fs::File, io::ErrorKind};

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
    let get_file_result4_1 = File::open("Hello.txt").expect("panic!!");
}
