use std::ops::Add;

// 泛型定义结构体，减少重复代码
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Clone,
{
    fn combine(&self) -> T {
        self.x.clone() + self.y.clone()
    }
}

#[derive(Debug)]
struct Score<X1, Y1> {
    name: X1,
    grade: Y1,
}

impl<X1, Y1> Score<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Score<X2, Y2>) -> Score<X1, Y2> {
        Score {
            name: self.name,
            grade: other.grade,
        }
    }
}

fn main() {
    // 重复代码合并
    let list_num = vec![34, 50, 25, 100, 65];
    let max_num = find_max_num(&list_num);
    println!("the largest number is {max_num}");
    let max_num_g = find_max(&list_num);
    println!("G the largest number is {max_num_g}");

    let list_char = vec!['d', 'r', 'e', 'w', 'a', 'i'];
    let max_char = find_max_char(&list_char);
    println!("the largest char is {max_char}");
    let max_char_g = find_max(&list_char);
    println!("G the largest number is {max_char_g}");

    // 结构体定义 枚举类亦可以
    let integer = Point { x: 12, y: 54 };
    println!("{:#?}", integer);
    println!("INT COMBINE IS {}", integer.combine());
    let float = Point { x: 34.3, y: 12.6 };
    println!("{:#?}", float);
    println!("FLOAT COMBINE IS {}", float.combine());

    let alice = Score {
        name: "Alice",
        grade: 9,
    };
    let bob = Score {
        name: "Bob",
        grade: "NINE",
    };
    println!("{:?}", alice);
    let alice2 = alice.mixup(bob);
    println!("{:?}", alice2)
}

fn find_max_num(list: &[i32]) -> &i32 {
    let mut max_num = &list[0];

    for num in list {
        if num > max_num {
            max_num = num
        }
    }
    max_num
}

fn find_max_char(list: &[char]) -> &char {
    let mut max_char = &list[0];

    for item in list {
        if item > max_char {
            max_char = item
        }
    }
    max_char
}

// 泛型合并重复代码
fn find_max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max_item = &list[0];

    for item in list {
        if item > max_item {
            max_item = item
        }
    }

    max_item
}
