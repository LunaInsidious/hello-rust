use std::fmt::Debug;

#[derive(Debug)]
struct Statistics {
  mean: f64,
  median: f64,
  mode: Option<i32>,
}

fn get_mean(numbers: &[i32]) -> f64 {
  let sum: i32 = numbers.iter().sum();
  sum as f64 / numbers.len() as f64
}

fn get_median(numbers: &[i32]) -> f64 {
  let mid = numbers.len() / 2;
  if numbers.len() % 2 == 0 {
    (numbers[mid - 1] + numbers[mid]) as f64 / 2.0
  } else {
    numbers[mid] as f64
  }
}

fn get_mode(numbers: &[i32]) -> Option<i32> {
  let mut counts = std::collections::HashMap::new();
  for &num in numbers {
    *counts.entry(num).or_insert(0) += 1;
  }
  // すべての要素が同じ回数の場合、Noneを返す
  if counts.len() == numbers.len() {
    return None;
  }
  let max_map = counts.iter().max_by_key(|&(_, count)| count);
  let (mode, _) = max_map.unwrap();
  Some(*mode)
}

fn get_integer_statistics(numbers: Vec<i32>) -> Statistics {
  let mean = get_mean(&numbers);
  let median = get_median(&numbers);
  let mode = get_mode(&numbers);
  Statistics { mean, median, mode }
}

pub fn practice1() {
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  let stats = get_integer_statistics(numbers);
  println!("Mean: {}", stats.mean);
  println!("Median: {}", stats.median);
  println!("Mode: {:?}", stats.mode);
  let numbers = vec![1, 2, 2, 3, 4, 5, 5, 6, 7, 8, 9];
  let stats = get_integer_statistics(numbers);
  println!("Mean: {}", stats.mean);
  println!("Median: {}", stats.median);
  println!("Mode: {:?}", stats.mode);
}

// 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
fn convert_to_big_laten(s: &String) -> String {
  let mut result = String::new();
  let mut chars = s.chars();
  let bowels = "aeiou";
  if let Some(first_char) = chars.nth(0) {
    if bowels.contains(first_char) {
      result = format!("{}-hay", s);
    } else {
      let first_consonant = &s[1..];
      result = format!("{}-{}ay", first_consonant, first_char);
    }
  }
  result
}

pub fn practice2() {
  let s = String::from("first");
  let result = convert_to_big_laten(&s);
  println!("result: {}", result);
  let s = String::from("apple");
  let result = convert_to_big_laten(&s);
  println!("result: {}", result);
  let s = String::new();
  let result = convert_to_big_laten(&s);
  println!("result: {}", result);
}

