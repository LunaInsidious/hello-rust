// 全参照にはライフタイムがあり、参照を使用する関数や構造体にはライフタイム引数を指定する必要がある
// ただし、ライフタイム省略規則により、ライフタイム引数を省略できる場合がある
// 1. 引数に複数の参照がある場合、すべての参照は個別のライフタイム引数を持つ
// 2. 引数に1つの参照がある場合、戻り値のライフタイムはその引数のライフタイムと同じになる
// 3. メソッドで、&selfまたは&mut selfを持つ場合、戻り値のライフタイムはselfのライフタイムと同じになる

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

pub fn function_lifetime() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");
  let result;
  {
    // ここでstring2を宣言すると、string2のスコープが短いのでエラーになる
    // let string2 = String::from("abcdefg");
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {}", string1);
  println!("The longest string is {}", string2);
  println!("The longest string is {}", result);
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Announcement! {}", announcement);
    self.part
  }
}

pub fn struct_lifetime() {
  let s: String = String::from("hello world");
  let r: &str = &s[0..5];
  let excerpt = ImportantExcerpt { part: r };
  println!("The excerpt is {}", excerpt.part);

  println!("The level is {}", excerpt.level());
  println!("The part is {}", excerpt.announce_and_return_part("hello"));
}

fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  announcement: T,
) -> &'a str
where
  T: std::fmt::Display,
{
  println!("Announcement! {}", announcement);
  if x.len() > y.len() { x } else { y }
}

pub fn integration_lifetime() {
  let string1 = String::from("abcd");
  let string2 = String::from("xyz");
  let result;
  {
    result = longest_with_an_announcement(&string1, &string2, "hello");
  }
  println!("The longest string is {}", string1);
  println!("The longest string is {}", string2);
  println!("The longest string is {}", result);
}
