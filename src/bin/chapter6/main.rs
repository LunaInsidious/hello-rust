mod enums;
mod if_lets;
mod matches;

fn main() {
  println!("enums call:");
  enums::main();
  enums::options();
  println!("\nmatches call:");
  matches::main();
  println!("\nif_lets call:");
  if_lets::main();
  println!("\nEnd of main function");
}
