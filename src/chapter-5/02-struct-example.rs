#[derive(Debug)] // 结构体定义之前加上外部属性
struct Rect {
    length: u32,
    height: u32,
    width: u32,
}

fn main() {
    // PART 1 例子：计算矩形面积
    let length = 30;
    let height = 20;

    let area1: u32 = calc_area1(length, height);
    println!("1 :The areas is {area1}");

    // PART 2 元组重构：计算矩形面积
    let rect2 = (30, 20);
    let area2: u32 = calc_area2(rect2);
    println!("2 :The areas is {area2}");

    // PART 3 结构体重构：计算矩形面积，提升代码可读性
    let vars = 2;
    let rect3 = Rect {
        length: 30,
        height: 20,
        // dbg! 返回表达式的值的所有权，
        // 所以 width 字段将获得相同的值，就像没有调用 dbg! 一样
        width: dbg!(30 * vars),
    };

    let area3: u32 = calc_area3(&rect3);
    println!("3 :The areas is {area3}");

    // 增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
    println!("rect is {:?}", rect3);
    println!("rect is {:#?}", rect3);
    dbg!(&rect3);
}

// 计算矩形面积
fn calc_area1(l: u32, h: u32) -> u32 {
    l * h
}

// 元组重构
fn calc_area2(lh: (u32, u32)) -> u32 {
    lh.0 * lh.1
}
// 结构体重构 提升可读性
fn calc_area3(rect: &Rect) -> u32 {
    rect.length * rect.height
}
