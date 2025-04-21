#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
  Colorado,
  Connecticut,
  Delaware,
  Florida,
  Georgia,
  Hawaii,
}

#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    }
  }
}

fn plus_one(x: Option<i32>) -> i32 {
  match x {
    Some(i) => i + 1,
    None => 0,
  }
}

fn some_u8_value(x: u8) {
  match x {
    3 => println!("three"),
    5 => println!("five"),
    _ => (),
  }
}

pub fn main() {
  let coin = Coin::Penny;
  println!("coin: {:?}", coin);
  println!("value_in_cents: {}", value_in_cents(coin));
  println!("value_in_cents: {}", value_in_cents(Coin::Nickel));
  println!("value_in_cents: {}", value_in_cents(Coin::Dime));
  println!(
    "value_in_cents: {}",
    value_in_cents(Coin::Quarter(UsState::Alabama))
  );
  println!(
    "value_in_cents: {}",
    value_in_cents(Coin::Quarter(UsState::Alaska))
  );

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("five: {:?}", five);
  println!("six: {:?}", six);
  println!("none: {:?}", none);

  some_u8_value(3u8);
  some_u8_value(0);
}
