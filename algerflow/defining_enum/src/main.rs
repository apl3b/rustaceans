#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        _ => {},
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(), 
    }

    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("{max}");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
        println!("{state:?}");
    } else {
        count += 1;
    }

    println!("{count}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
