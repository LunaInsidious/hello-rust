use std::collections::HashMap;

fn create_from_vec() {
  let teams = vec![
    String::from("Blue"),
    String::from("Yellow"),
    String::from("Red"),
    String::from("Green"),
  ];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  println!("scores: {:?}", scores);
}

pub fn map_creation() {
  let mut scores = HashMap::new();

  let field1 = String::from("Blue");
  let field2 = String::from("Yellow");
  let value1 = String::from("10");
  let value2 = String::from("50");
  scores.insert(field1, &value1);
  scores.insert(field2, &value2);
  println!("scores: {:?}", scores);

  //   String値のinsertはムーブされるので使えない
  //   println!("field1: {}", field1);
  //   println!("field2: {}", field2);
  println!("value1: {}", value1);
  println!("value2: {}", value2);

  create_from_vec();
}

pub fn map_access() {
  let mut scores = HashMap::new();
  let field1 = String::from("Blue");
  let field2 = String::from("Yellow");
  let value1 = String::from("10");
  let value2 = String::from("50");
  scores.insert(&field1, value1);
  scores.insert(&field2, value2);

  let score = scores.get(&field1);
  println!("blue score: {:?}", score);

  let score = scores.get(&String::from("Unknown"));
  println!("unknown score: {:?}", score);
}

pub fn map_update() {
  let mut scores = HashMap::new();
  let field1 = String::from("Blue");
  let field2 = String::from("Yellow");
  scores.insert(&field1, 10);
  // valueは上書きされる
  scores.insert(&field1, 50);

  println!("blue score: {:?}", scores);

  scores.entry(&field2).or_insert(20);
  scores.entry(&field2).or_insert(30);
  println!("scores: {:?}", scores);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("word count: {:?}", map);
}
