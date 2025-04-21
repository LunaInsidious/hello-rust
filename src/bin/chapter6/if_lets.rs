// 1つの値にだけmatchしたい場合のコード
fn match_one_value(x: Option<u8>) {
  match x {
    Some(3) => println!("three"),
    _ => (),
  }
}

// if letは値が一つのパターンにマッチしたときにコードを走らせ、他は無視するmatchへの糖衣構文
fn if_one_value(x: Option<u8>) {
  if let Some(3) = x {
    println!("three");
  } else {
    println!("not three");
  }
}

pub fn main() {
  let x = Some(3);
  match_one_value(x);
  if_one_value(x);
}
