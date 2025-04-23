pub fn vector_creation() {
  // ベクタの作成
  let v: Vec<i32> = Vec::new();
  println!("v: {:?}", v);
  let v = vec![1, 2, 3];
  println!("v: {:?}", v);
  let mut v = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  println!("v: {:?}", v);
}

pub fn vector_access() {
  let mut v = vec![1, 2, 3];
  let third = v[2];
  println!("third: {}", third);
  let third = &v[2];
  println!("third: {}", third);
  match v.get(100) {
    Some(x) => println!("third: {}", x),
    None => println!("There is no 100th element."),
  }

  //   不変な参照(third)がまだ有効なので、可変な参照は作れない
  //   v.push(4);
  //   println!("third: {}", third);

  let third = v[2];

  v.push(2);
  // このthirdは新しくスタック上に作られた参照なのでOK
  println!("third: {}", third);
}

pub fn vector_iteration() {
  let v = vec![1, 2, 3, 4, 5];
  for i in &v {
    println!("i: {}", i);
  }

  let mut v = vec![1, 2, 3, 4, 5];
  for i in &mut v {
    *i += 50;
  }
  println!("Updated vector: {:?}", v);
}

// enumを使うことで異なる型の値を持つことができる
#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn vector_enum() {
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("blue")),
  ];
  println!("row: {:?}", row);
  for cell in &row {
    match cell {
      SpreadsheetCell::Int(i) => println!("Int: {}", i),
      SpreadsheetCell::Float(f) => println!("Float: {}", f),
      SpreadsheetCell::Text(s) => println!("Text: {}", s),
    }
  }
}
