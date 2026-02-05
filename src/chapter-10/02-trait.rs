fn main() {
    let news1 = News {
        title: String::from("US & UN & UK"),
        time: String::from("2026-02-05"),
        location: String::from("CA.US"),
        content: String::from("nothing"),
    };

    println!("there is new -- {}", news1.summarize());
    println!("{}", news1.summarize1());
    post(&news1);
    post_bound(&news1);
    post_bound_more(&news1, "Oha");
    let new2 = return_summerize(); //应用了trait的实例数据
    println!(" {:?}", new2.summarize1());
}

// lib.rs
pub trait Summary {
    fn summarize(&self) -> String;

    // 如重写即覆盖默认实现
    fn summarize1(&self) -> String {
        // 默认实现
        String::from("(Read more...)")
    }
}

// 在类型上实现trait
pub struct News {
    pub title: String,
    pub time: String,
    pub location: String,
    pub content: String,
}

// 应用
impl Summary for News {
    //重写实现-
    fn summarize(&self) -> String {
        format!("{}, in {}({})", self.title, self.location, self.time)
    }
}

// trait作为参数
pub fn post(item: &impl Summary) {
    println!("Big NEWS!!{}", item.summarize())
}

// trait bound语法（trait结合泛型）impl Trait的语法糖
pub fn post_bound<T: Summary>(item: &T) {
    println!("Big NEWS!!{}", item.summarize())
}

pub fn post_bound_more<T: Summary>(item: &T, str: &str) {
    println!("{str}, Big NEWS!!{}", item.summarize())
}

// trait作为返回
fn return_summerize() -> impl Summary {
    News {
        title: String::from("return US & UN & UK"),
        time: String::from("2026-02-04"),
        location: String::from("return CA.US"),
        content: String::from("return nothing"),
    }
}
