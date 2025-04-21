#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

pub fn user() {
  let mut user1 = build_user(String::from("user1@example.com"), String::from("user1"));

  println!("user1: {:?}", user1);

  user1.username = String::from("user1_updated");
  user1.email = String::from("user1_updated@example.com");
  user1.sign_in_count += 1;
  user1.active = true;

  println!("user1: {:?}", user1);

  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("another"),
    ..user1
  };

  println!("user2: {:?}", user2);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: false,
    sign_in_count: 1,
  }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn tuple_struct() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("black: {:?}", black);
  println!("origin: {:?}", origin);

  let Color(r, g, b) = black;
  println!("r: {}, g: {}, b: {}", r, g, b);

  let Point(x, y, z) = origin;
  println!("x: {}, y: {}, z: {}", x, y, z);
}

pub fn unit_struct() {
  #[derive(Debug)]
  struct UnitStruct;
  let unit = UnitStruct;
  println!("Unit struct: {:?}", unit);
}
