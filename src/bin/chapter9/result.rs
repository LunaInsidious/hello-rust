use std::io::{Error, Read};
use std::{fs::File, io::ErrorKind};

pub fn file() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(ref e) if e.kind() == ErrorKind::NotFound => {
      println!("File not found, creating a new one.");
      match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      }
    }
    Err(e) => {
      panic!("Problem opening the file: {:?}", e);
    }
  };

  println!("File opened successfully: {:?}", f);
}

fn read_username_from_file() -> Result<String, Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::from("username");

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

fn read_username_from_file_short() -> Result<String, Error> {
  let mut s = String::from("username");
  // ?演算子はエラーの場合from関数を使って、現在の関数の戻り値の型に変換する
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

pub fn transfer() {
  match read_username_from_file() {
    Ok(username) => println!("Username: {}", username),
    Err(e) => println!("Error reading username: {:?}", e),
  }
  match read_username_from_file_short() {
    Ok(username) => println!("Username: {}", username),
    Err(e) => println!("Error reading username: {:?}", e),
  }
}
