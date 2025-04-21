pub fn string_example() {
  let s = "hello"; // スタックに格納される
  println!("{}", s);

  let mut s = String::from("hello"); // ヒープに格納される
  s.push_str(", world!");
  println!("{}", s);

  //   let s1 = String::from("hello");
  //   let s2 = s1; // 静的エラー。borrow of moved value: `s1`
  //   println!("s1 = {}, s2 = {}", s1, s2);

  let s = 5;
  let y = s; // スタックに格納されるので、s1をmoveする必要がない
  println!("s = {}, y = {}", s, y);
}

pub fn heap_move() {
  let s = String::from("hello"); // sがスコープに入る

  takes_ownership(s); // sの値が関数にムーブされ...
  // ... ここではもう有効ではない
  //   println!("{}", s); // ここでエラーになる。sはムーブされているので、もう使えない

  let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする
  println!("{}", s1); // s1は有効

  let s2 = String::from("hello"); // s2がスコープに入る

  let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
  //   println!("{}", s2); // ここでエラーになる。s2はムーブされているので、もう使えない
  println!("{}", s3); // s3は有効
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、何も起きない。s1もスコープを抜け、ドロップされる。

fn takes_ownership(some_string: String) {
  // some_stringがスコープに入る。
  println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn gives_ownership() -> String {
  // gives_ownershipは、戻り値を
  // 呼び出した関数にムーブする

  let some_string = String::from("hello"); // some_stringがスコープに入る

  some_string // some_stringが返され、呼び出し元関数にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
  // a_stringがスコープに入る。

  a_string // a_stringが返され、呼び出し元関数にムーブされる
}

pub fn stack_move() {
  let x = 5; // xがスコープに入る

  makes_copy(x); // xも関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫
  println!("{}", x); // ここではxは有効
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn makes_copy(some_integer: i32) {
  // some_integerがスコープに入る
  println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

pub fn reusable_ownership() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
