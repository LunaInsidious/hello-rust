struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f64> {
  fn distance_from_origin(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

pub fn main() {
  let p = Point { x: 5, y: 10 };
  println!("x: {}", p.x());
  println!("y: {}", p.y);
  let p2 = Point { x: 3.0, y: 4.0 };
  println!("distance from origin: {}", p2.distance_from_origin());
}
