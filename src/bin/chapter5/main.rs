mod examples;
mod methods;
mod structs;

fn main() {
  println!("structs call:");
  structs::user();
  structs::tuple_struct();
  structs::unit_struct();
  println!("\nexamples call:");
  examples::main();
  println!("\nmethods call:");
  methods::main();
  println!("\nEnd of main function");
}
