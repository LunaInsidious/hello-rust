pub fn slice() {
  let s = String::from("hello world!");

  let slice1 = &s[0..5];
  let slice2 = &s[..5];
  println!("{} = {}", slice1, slice2);
  let len = s.len();

  let slice3 = &s[5..];
  let slice4 = &s[5..len];
  println!("{} = {}", slice3, slice4);

  let slice5 = &s[..];
  let slice6 = &s[0..len];
  println!("{} = {}", slice5, slice6);

  let num_s = [1, 2, 3, 4, 5];
  let slice7 = &num_s[1..3];
  println!("slice7: {:?}", slice7);
}

pub fn first_word_example() {
  let mut s = String::from("hello world!");
  let word = first_word(&s);
  println!("The first word is: {}", word);

  //   s.clear(); // エラーになる。sは不変な参照を持っているので、変更できない
  println!("s: {} ,word: {}", s, word);

  s.clear(); // wordはスコープを抜けているので、sを変更できる
  println!("s: {}", s);

  let s_literal = "hello world!";
  let word_literal = first_word(s_literal);
  println!("The first literal word is: {}", word_literal);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}
