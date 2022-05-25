fn main() {
    println!("Hello, world!");
    loop_demo();
    nested_loop_demo();
    for_loop_demo();
    while_loop_demo();
}

fn loop_demo() {
    let mut count = 5;
    loop {
        println!("Count is {}", count);
        count -= 1;

        if count == 0 {
            break;
        }
    }
}

fn nested_loop_demo() {
    let mut count = 0;
    'count_up_label: loop {
        println!("Count is {}", count);
        let mut nested_count = 0;
        loop {
            nested_count += 1;
            println!("Nested count is {}", nested_count);
            if nested_count == 5 {
                break;
            }
            if count == 5 {
                break 'count_up_label;
            }
        }
        count += 1;
    }
    println!("Welcome back");
}

fn for_loop_demo() {
    let a = [1, 2, 3, 4];

    for (index, element) in a.iter().enumerate() {
        println!("a[{index}] = {}", element);
    }

    for i in (1..4).rev() {
        println!("i = {}", i);
    }
}

fn while_loop_demo() {
    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < 5 {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }
}
