pub fn if_condition() {
  let number = 6;

  if number < 5 {
    println!("condition was true");
  } else if number == 5 {
    println!("condition was equal to 5");
  } else {
    println!("condition was false");
  }

  let condition = true;
  // ifは式なので、値を返すことができる
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {}", number);
}

pub fn loop_control() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {}", count);
    let mut remaining = 10;

    loop {
      println!("remaining = {}", remaining);
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count = {}", count);
}

pub fn while_control() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF!");

  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }
}

pub fn for_control() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!");
}
