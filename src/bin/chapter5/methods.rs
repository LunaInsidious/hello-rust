#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // メソッドを定義する
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    // 呼び出し側は自動参照してくれるので、これで問題ない
    rect1.area()
  );

  println!("rect1 is {:?}", rect1);
  println!("rect1 is {:#?}", rect1);

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", (&rect1).can_hold(&rect3));

  let square = Rectangle::square(10);
  println!("square: {:?}", square);
}
