fn main() {
    // PART 1 变量作用域
    // vars 还未声明，无效
    {
        let vars: &'static str = "scope"; // 从此开始 str有效
        println!("{vars} is valid"); // 使用str
    } // 作用域结束，vars无效

    // println!("{vars} is unvaild"); // 作用域结束，vars无效,报错

    // PART 2 String类型数据
    // String 类型存在栈内的是对应变量的【指针-pointer】、【长度-length】和【容量-capacity】
    // String 类型存在堆内的是对应变量的数据内容
    // String 类型的内存有效期即使指针ptr的内存有效期
    let mut str: String = String::from("hello, ");
    println!("init {str}");
    // 指针地址
    println!("{str},pointer- {:p}", str.as_ptr());
    // 变量值的有效字节数 1 char占一个字节
    println!("{str},length- {}", str.len());
    // 变量在堆内的内存容量（可分配的最大字节数）
    // 一般来说 cap >= len

    println!("{str},capacity- {}", str.capacity());
    str.push_str("world!");
    println!("push {str}");
    println!("{str},pointer- {:p}", str.as_ptr());
    println!("{str},length- {}", str.len());
    println!("{str},capacity- {}", str.capacity());

    // capacity自动扩容：len > cap, cap = cap*2, 容量翻倍
    let mut cap_str = String::with_capacity(8);
    cap_str.push_str("one,two,");
    println!("{cap_str},length- {}", cap_str.len());
    println!("{cap_str},capacity- {}", cap_str.capacity());
    cap_str.push_str("three");
    println!("{cap_str},length- {}", cap_str.len());
    println!("{cap_str},capacity- {}", cap_str.capacity());

    // PART 3 内存和分配
    // 无需内存分配器分配，指定了所占内存大小，用栈的形式存储，x和y
    let x = 5;
    let y = x;
    println!("x={x}, y={y}");

    let str1: String = String::from("STR1-VALUE");
    let ptr = str1.as_ptr();
    let len = str1.len();
    let cap = str1.capacity();
    println!("{str1},{:p}, {}, {}", ptr, len, cap);

    // unsafe {
    //     println!("first char:{}", *ptr as char); // S
    //     println!("first char as ascii: {}", *ptr); // 83
    // }

    // String 类型是以堆的形式存储的，使用者无法明确String类型数据所占内存大小
    //  只拷贝了栈里的数据，即【指针-pointer】、【长度-length】和【容量-capacity】
    //  并没有拷贝 堆内数据
    let str2: String = str1;
    // 报错-str1被销毁
    // 这是因为如果str1 和str2离开作用域，销毁时会释放堆内相同的内存，
    // 会导致二次释放的错误，
    // 所以rust规定str1的栈内数据失效被销毁 （所有者规则之二， 值任何情况下有且只有一个所有者）
    // 这个操作多于浅拷贝，称之为【移动】
    // println!("str1={str1}, str2={str2}");
    println!("str2={str2}");

    // PART 4 作用域与赋值
    let mut str4 = String::from("scope and value");
    str4 = String::from("copy scope and value");
    println!("str4 is {str4}");

    // PART 5 克隆与数据交互
    let str5 = String::from("fotr clone");
    let str5_clone = str5.clone();
    println!("str5={str5}, str5_clone={str5_clone}");
    println!("{}", str5 == str5_clone);
    println!("{}", str5.as_ptr() == str5_clone.as_ptr());

    // PART 6 所有权 与函数
    let str6 = String::from("for function");

    fn takes_ownship(str: String) {
        println!("{str},this string takes ownship");
    } //移动参数内, 调用drop, 数据失效

    takes_ownship(str6);
    // 报错
    // println!("{str6}, this string is not valid");

    let var6 = 5;
    makes_copy(var6);

    fn makes_copy(int: i32) {
        println!("{int}, this interger makes copy");
    }

    println!("{var6}, this interger is still valid");

    // PART 7 返回值与作用域
    let str7 = return_ownship1();
    println!("{str7}");

    fn return_ownship1() -> String {
        let s = String::from("HEY!!I AM BACK!");
        s
    }

    let str7_1 = String::from("I AM BACK TOO!!!");
    let str7_2 = takes_and_return_ownships(str7_1);
    // str7_1 无效了，移动给了str7_2
    println!("{str7_2}");

    fn takes_and_return_ownships(str: String) -> String {
        str
    }

    let str7_3 = takes_and_change_and_return_ownships(str7_2);
    println!("{str7_3}");

    fn takes_and_change_and_return_ownships(mut str: String) -> String {
        str.push_str("HEY!!");
        str
    }
}
