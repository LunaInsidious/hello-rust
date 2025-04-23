mod generics;
mod lifetimes;
mod traits;

fn main() {
  println!("generics call:");
  generics::main();
  println!("\ntraits call:");
  traits::traits();
  traits::largest_trait();
  traits::impl_trait();
  println!("\nlifetimes call:");
  lifetimes::function_lifetime();
  lifetimes::struct_lifetime();
  lifetimes::integration_lifetime();
  println!("\nEnd of main function");
}
