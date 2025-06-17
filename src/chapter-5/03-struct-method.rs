#[derive(Debug)] // 结构体定义之前加上外部属性
struct Rect {
    length: u32,
    height: u32,
}

impl Rect {
    // &self 等效于 &Rectangle
    fn calc_area(&self) -> u32 {
        self.length * self.height
    }

    fn calc_area_mut(&mut self) -> u32 {
        self.height = self.height * 2;
        self.length * self.height
    }

    fn height(&self) -> bool {
        self.height > 30
    }

    fn can_cover(&self, comp: &Rect) -> bool {
        self.length > comp.length && self.height > comp.height
    }

    fn be_square(size: u32) -> Self {
        Self {
            length: size,
            height: size,
        }
    }
}

impl Rect {
    fn be_square_two(size: u32) -> Self {
        Self {
            length: size * 2,
            height: size * 2,
        }
    }
}

fn main() {
    // PART 1 impl的应用
    let mut rect1 = Rect {
        length: 30,
        height: 20,
    };

    let area1 = rect1.calc_area();
    println!("1-1, area1={area1}");
    println!("1-1, area1={rect1:#?}");
    if !rect1.height() {
        println!("rect1.height is not over 30");
    }
    let area2 = rect1.calc_area_mut();
    println!("1-2, area2={area2}");
    println!("1-2, area1={rect1:#?}");

    if rect1.height() {
        println!("rect1.height is over 30");
        println!("rect1.length is {}", rect1.length);
        // println!("rect1.length is {}", rect1.length());
    }
    // PART 2 MORE
    let rect2 = Rect {
        length: 30,
        height: 20,
    };

    let rect3 = Rect {
        length: 40,
        height: 30,
    };
    let rect4 = Rect {
        length: 20,
        height: 10,
    };

    println!("Can rect2 cover rect3? {}", rect2.can_cover(&rect3));
    println!("Can rect2 cover rect4? {}", rect2.can_cover(&rect4));

    // PART 3 关联函数 Impl中不以 self 为第一参数的关联函数（不是方法）
    let sq = Rect::be_square(25);
    println!("sq is {:?}", sq);
    // PART 4 多个impl块
    let sq2 = Rect::be_square_two(25);
    println!("sq2 is {:?}", sq2);
}
