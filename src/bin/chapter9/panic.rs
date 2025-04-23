pub fn self_panic() {
  panic!("crash and burn");
}

pub fn vec_panic() {
  let v = vec![1, 2, 3];
  println!("v[99]: {}", v[99]);
}
