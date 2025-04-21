#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

impl IpAddr {
  fn call(&self) {
    match self {
      IpAddr::V4(a, b, c, d) => println!("call v4: {}.{}.{}.{}", a, b, c, d),
      IpAddr::V6(addr) => println!("call v6: {}", addr),
    }
  }
}

pub fn main() {
  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));
  println!("{:?}", home);
  println!("{:?}", loopback);

  home.call();
  loopback.call();
}

pub fn options() {
  let some_number = Some(5);
  let some_string = Some("a string");
  // Noneは型を指定しないとエラーになる
  let absent_number: Option<i32> = None;
  println!("{:?}", some_number);
  println!("{:?}", some_string);
  println!("{:?}", absent_number);

  let x = 5;
  let y = Some(5);
  // Option<T>のnullは他の言語でいうところのnullと同じ
  let sum = x + y.unwrap_or(0);
  println!("sum: {}", sum);
}
