pub fn string_creation() {
  let mut s = String::new();
  println!("s: {}", s);

  s = "initial contents".to_string();
  println!("{}", s);

  s = String::from("initial contents");
  println!("{}", s);

  println!("{}", String::from("السلام عليكم"));
  println!("{}", String::from("Dobrý den"));
  println!("{}", String::from("Hello"));
  println!("{}", String::from("שָׁלוֹם"));
  println!("{}", String::from("नमस्ते"));
  println!("{}", String::from("こんにちは"));
  println!("{}", String::from("안녕하세요"));
  println!("{}", String::from("你好"));
  println!("{}", String::from("Olá"));
  println!("{}", String::from("Здравствуйте"));
  println!("{}", String::from("Hola"));

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  // push_strは所有権を奪わないので、s2はまだ使える
  println!("{}", s2);

  let s3 = String::from("hello");
  let s4 = String::from("world");
  //   +演算子で呼び出されるaddメソッドは、参照外し型強制を行う
  let s5 = s3 + &s4;
  //   addメソッドの引数側はポインタでなければならないため、以下はエラー
  //   let s6 = s3 + s4;

  //   s3は所有権を奪われたので、s3は使えない
  //   println!("{}", s3);
  //   s4はまだ使える
  println!("{}", s4);
  println!("{}", s5);

  let tic = String::from("Tic");
  let tac = String::from("Tac");
  let toe = String::from("Toe");
  let s = format!("{}-{}-{}", tic, tac, toe);
  println!("{}", s);
}

pub fn string_access() {
  let s1 = String::from("hello");
  //   stringは添え字アクセス出来ない。
  // マルチバイト文字の場合、O(1)でアクセスできないため
  //   println!("s1: {}", &s1[0]);
  println!("s1: {}", &s1.bytes().nth(0).unwrap());
  let s1 = String::from("ありがとう🍺");
  println!("s: {}, len: {}", s1, s1.len());
  let s2 = &s1[0..3];
  println!("{}", s2);
  let s3 = &s1[15..];
  println!("{}", s3);
  // マルチバイト文字で、1文字を表していないスライスは参照できない
  let s4 = if s1.is_char_boundary(2) {
    Some(&s1[2..])
  } else {
    println!("Invalid index");
    None
  };
  println!("{:?}", s4);

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
