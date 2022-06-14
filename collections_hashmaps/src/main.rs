use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores2.entry(String::from("Blue")).or_insert(20);
    scores2.entry(String::from("Pink")).or_insert(20);

    println!("{:?}", scores2);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // count mutable value of 'word' key
        *count += 1; 
    }

    println!("{:?}", map);
}
