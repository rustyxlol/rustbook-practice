enum Cells {
    Int(i32),
    Text(String),
}


fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(10);
    v.push(20);

    let element = &v[0];

    println!("{}", element);



    match v.get(1) {
        Some(number) => println!("Here's your number boss: {}", number),
        None => println!("Seems like it does not exist")
    }

    for i in &mut v {
        *i += 10;
    }

    let row = vec![Cells::Int(3), Cells::Text((String::from("Nice")))];
}
