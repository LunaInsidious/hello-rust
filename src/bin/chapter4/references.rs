pub fn reference() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1); // 引数に参照を渡す
  println!("The length of '{}' is {}.", s1, len); // s1は有効

  let r1 = &s1; // 不変な参照を作成
  let r2 = &s1; // 不変な参照を作成
  println!("{} and {}", r1, r2); // 不変な参照は同時に複数作成できる
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

pub fn mutable_reference() {
  let mut s1 = String::from("hello");
  //   let r1 = &mut s1;
  //   let r2 = &mut s1; // エラーになる。r1がスコープを抜けるまで、r2は作成できない
  //   println!("{} and {}", r1, r2);
  change(&mut s1); // 引数に可変参照を渡す
  println!("After change: {}", s1);

  {
    let r1 = &mut s1; // 可変参照を作成
    println!("r1: {}", r1);
  }
  let r2 = &mut s1; // r1がスコープを抜けたので、r2を作成できる
  println!("r2: {}", r2);

  //   不変な参照と可変参照は同時に作成できない
  //   let r3 = &s1; // 不変な参照を作成
  //   let r4 = &mut s1; // エラーになる。r3がスコープを抜けるまで、r4は作成できない
  //   println!("{} and {}", r3, r4); // 不変な参照と可変参照は同時に作成できない
}

fn change(some_string: &mut String) {
  some_string.push_str(", world!"); // ここでエラーになる。some_stringは不変な参照なので、変更できない
}

// fn dangle() -> &String {
//   let s = String::from("hello"); // sがスコープに入る
//   &s // sの参照を返すが、sはスコープを抜けるとドロップされる
// } // ここでsがスコープを抜け、ドロップされる。rはsの参照を持っているが、sはドロップされているので、エラーになる

pub fn no_dangle_main() {
  let r = no_dangle(); // rはno_dangleの戻り値を持つ
  println!("r: {}", r); // rは有効
}

fn no_dangle() -> String {
  let s = String::from("hello"); // sがスコープに入る
  s // sを返す。sはスコープを抜けるが、ムーブされるので、ドロップされない
} // ここでsがスコープを抜け、ドロップされる。rはsの値を持っているので、エラーにならない
