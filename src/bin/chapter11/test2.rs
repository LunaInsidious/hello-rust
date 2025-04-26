#[cfg(test)]
mod tests {
  #[test]
  fn this_test_will_pass() {
    // cargo test時には出力されない
    // cargo test -- --nocaptureで出力される
    println!("----------PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP------------------");
    assert_eq!(2 + 2, 4);
  }
  #[test]
  fn this_test_will_fail() {
    println!(
      "----------FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF------------------"
    );
    // assert_eq!(2 + 2, 5);
  }

  #[test]
  #[ignore]
  fn expensive_test() {
    // cargo test -- --ignoredで実行される
    println!("----------IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII------------------");
    assert_eq!(2 + 1, 4);
  }
}
