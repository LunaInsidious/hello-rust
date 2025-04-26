#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
  width: u32,
  height: u32,
}

#[allow(dead_code)]
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

impl PartialEq for Rectangle {
  fn eq(&self, other: &Self) -> bool {
    self.width * self.height == other.width * other.height
  }
}

#[cfg(test)]
mod tests1 {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn it_works2() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err("Test failed".to_string())
    }
  }

  #[test]
  #[should_panic]
  fn another() {
    panic!("Make this test fail!");
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = super::Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = super::Rectangle {
      width: 5,
      height: 1,
    };
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = super::Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = super::Rectangle {
      width: 5,
      height: 1,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn equality() {
    let rect1 = super::Rectangle {
      width: 4,
      height: 5,
    };
    let rect2 = super::Rectangle {
      width: 10,
      height: 2,
    };
    assert_eq!(rect1, rect2);
  }

  #[test]
  fn not_equal() {
    let rect1 = super::Rectangle {
      width: 4,
      height: 5,
    };
    let rect2 = super::Rectangle {
      width: 10,
      height: 3,
    };
    assert_ne!(rect1, rect2);
  }

  #[test]
  #[should_panic]
  fn error_message() {
    let left = "Hello";
    let right = "World";
    assert!(
      left == right,
      "The strings are not equal. Expected {} but got {}",
      left,
      right
    );
  }
}

#[allow(dead_code)]
pub struct Guess {
  value: i32,
}

#[allow(dead_code)]
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      //予想値は1から100の間でなければなりませんが、{}でした。
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }
}

#[cfg(test)]
mod tests2 {
  use super::*;

  #[test]
  fn test_guess() {
    let guess = Guess::new(50);
    assert_eq!(guess.value, 50);
  }

  #[test]
  #[should_panic(expected = "Guess value must be between 1 and 100, got -10.")]
  fn test_guess_out_of_bounds() {
    Guess::new(-10);
  }
}
