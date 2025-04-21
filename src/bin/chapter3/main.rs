mod controls;
mod functions;
mod types;
mod variables;

fn main() {
  println!("variables call:");
  variables::mutable();
  variables::constant();
  variables::shadowing();
  println!("\ntypes call:");
  types::integers();
  types::floating_point();
  types::boolean();
  types::character();
  types::tuple();
  types::array();
  println!("\nfunctions call:");
  functions::main();
  println!("\ncontrols call:");
  controls::if_condition();
  controls::loop_control();
  controls::while_control();
  controls::for_control();
  println!("\nEnd of main function");
}
