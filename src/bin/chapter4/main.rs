mod ownerships;
mod references;
mod slices;

fn main() {
  println!("ownerships call:");
  ownerships::string_example();
  ownerships::heap_move();
  ownerships::stack_move();
  ownerships::reusable_ownership();
  println!("\nreferences call:");
  references::reference();
  references::mutable_reference();
  references::no_dangle_main();
  println!("\nslices call:");
  slices::slice();
  slices::first_word_example();
  println!("\nEnd of main function");
}
