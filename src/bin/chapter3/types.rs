pub fn integers() {
  // 各整数型の最大値と最小値を表示
  println!("i8  : min = {}, max = {}", i8::MIN, i8::MAX);
  println!("u8  : min = {}, max = {}", u8::MIN, u8::MAX);
  println!("i16 : min = {}, max = {}", i16::MIN, i16::MAX);
  println!("u16 : min = {}, max = {}", u16::MIN, u16::MAX);
  println!("i32 : min = {}, max = {}", i32::MIN, i32::MAX);
  println!("u32 : min = {}, max = {}", u32::MIN, u32::MAX);
  println!("i64 : min = {}, max = {}", i64::MIN, i64::MAX);
  println!("u64 : min = {}, max = {}", u64::MIN, u64::MAX);
  println!("isize: min = {}, max = {}", isize::MIN, isize::MAX);
  println!("usize: min = {}, max = {}", usize::MIN, usize::MAX);

  // オーバーフローの例（ラップアラウンド）
  let a: u8 = 255;
  let b = a.wrapping_add(1);
  println!("255u8 + 1 (wrapping_add) = {}", b); // 0

  // チェック付き加算（オーバーフロー時はNone）
  let c = 255u8.checked_add(1);
  println!("255u8 + 1 (checked_add) = {:?}", c); // None

  // 飽和加算（オーバーフロー時は最大値）
  let d = 255u8.saturating_add(1);
  println!("255u8 + 1 (saturating_add) = {}", d); // 255

  // 型変換（キャスト）の例
  let e: i16 = -1;
  let f = e as u16;
  println!("-1i16 as u16 = {}", f); // 65535

  // 数値リテラルの例
  let decimal = 98_222;
  let hex = 0xff;
  let octal = 0o77;
  let binary = 0b1111_0000;
  // Ascii文字のバイト値
  // 文字リテラルはu8型に変換される
  let byte = b'A';

  println!("decimal = {}", decimal);
  println!("hex = {}", hex);
  println!("octal = {}", octal);
  println!("binary = {}", binary);
  println!("byte = {}", byte);
}

pub fn floating_point() {
  println!("f32 : min = {}, max = {}", f32::MIN, f32::MAX);
  println!("f64 : min = {}, max = {}", f64::MIN, f64::MAX);

  let sum = 5.0 + 10.2;
  println!("5.0 + 10.2 = {}", sum);
  let difference = 95.5 - 4.3;
  println!("95.5 - 4.3 = {}", difference);
  let product = 4.0 * 30.0;
  println!("4.0 * 30.0 = {}", product);
  let quotient = 56.7 / 32.2;
  println!("56.7 / 32.2 = {}", quotient);
  let floored = 2 / 3;
  println!("2 / 3 = {}", floored);
  let floored_float = 2.0 / 3.0;
  println!("2.0 / 3.0 = {}", floored_float);
  let remainder = 43 % 5;
  println!("43 % 5 = {}", remainder);
}

pub fn boolean() {
  let t = true;
  let f: bool = false;
  println!("t = {}, f = {}", t, f);
}

pub fn character() {
  let c = 'z';
  let z: char = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}

pub fn tuple() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (_, y, _) = tup;
  println!("y = {}", y);
  let (x, y, z) = tup;
  println!("x = {}, y = {}, z = {}", x, y, z);
  println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
}

pub fn array() {
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
  println!("first = {}, second = {}", first, second);
  println!("a[0] = {}, a[1] = {}", a[0], a[1]);

  let a = [3; 5]; // [3, 3, 3, 3, 3]
  println!("a = {:?}", a);
}
