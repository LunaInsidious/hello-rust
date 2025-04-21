pub fn main() {
  println!("Hello, world!");

  print_labeled_measurement(5, 'h');

  let y = {
    let x = 3;
    // 式の終端にはセミコロンを付けない
    // 付けると式ではなく文になり、値を返さない
    x + 1
  };
  println!("The value of y is: {}", y);

  let x = five();
  println!("The value of x is: {}", x);

  let x = plus_one(x);
  println!("The value of x after plus_one is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The value of x is: {}{}", value, unit_label);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
