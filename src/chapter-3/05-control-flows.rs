fn main() {
    // PART 1 if else
    let n = 5;
    if n > 5 {
        println!("it is true");
    } else {
        println!("it is false");
    }

    if n % 3 == 0 {
        println!("it is divisible by 3");
    } else if n % 2 == 0 {
        println!("it is divisible by 2");
    } else if n % 1 == 0 {
        println!("it is divisible by 1");
    } else {
        println!("it is not divisible by 1, 2, or 3");
    }

    let cond = if n > 5 { n } else { n + 1 };
    println!("cond is {cond}");

    // PART 2 循环
    // loop
    let mut loop_num = 0;
    loop {
        loop_num += 1;
        println!("loop, number{}", loop_num);
        if loop_num > 4 {
            break;
        }
    }

    let mut loop_cnt = 0;
    let loop_result = loop {
        loop_cnt += 1;

        if loop_cnt == 4 {
            break loop_cnt * 2;
        }
    };
    println!("the loop result is {loop_result}");

    // loop in loop

    let mut outer_cnt = 0;
    'cnt_loop: loop {
        println!("oooooooooouter_count = {outer_cnt}");
        let mut inner_cnt = 10;

        loop {
            if outer_cnt > 5 {
                break 'cnt_loop;
            }
            // if inner_cnt == 8 {
            //     outer_cnt += 2;
            //     continue;
            // }
            if inner_cnt == 6 {
                break;
            }

            inner_cnt -= 1;
        }

        outer_cnt += 1;
    }
    println!("final outer_cnt is {outer_cnt}");

    // while
    let mut while_num = 3;
    while while_num != 0 {
        println!("while_num is {while_num}");

        while_num -= 1
    }
    println!("while end!!");

    let for_arr: [i32; 6] = [1, 2, 3, 4, 23, 1];
    let mut idx: usize = 0;
    while idx < 6 {
        println!("while: the value in {idx} is {}", for_arr[idx]);
        idx += 1;
    }

    // while idx < 6 {
    //     if idx > 3 {
    //         break;
    //     }
    //     println!("while: the value in {idx} is {}", for_arr[idx]);
    //     idx += 1;
    // }

    // for 避免了显式的数组长度的约束
    let mut for_idx = 0;
    for ele in for_arr {
        println!("for: the value in {for_idx} is {}", ele);
        for_idx += 1;
    }
}
