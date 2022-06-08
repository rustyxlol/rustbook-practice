#[derive(Debug)]
enum UsState {
    Alaska,
    Washington,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn coin_value(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Quarter of: {:?}", state);
                25
            }
        }
    }
}

fn main() {
    println!("{}", Coin::coin_value(Coin::Penny));
    println!("{:?}", Coin::coin_value(Coin::Nickle));
    println!("{:?}", Coin::coin_value(Coin::Dime));
    println!("{:?}", Coin::coin_value(Coin::Quarter(UsState::Alaska)));
    println!("{:?}", Coin::coin_value(Coin::Quarter(UsState::Washington)));
}
