mod panic;
mod result;

fn main() {
  println!("panic call:");
  panic::self_panic();
  panic::vec_panic();
  println!("\nresult call:");
  result::file();
  result::transfer();
  println!("\nEnd of main function");
}
