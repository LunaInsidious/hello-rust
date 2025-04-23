mod hash_maps;
mod practices;
mod strings;
mod vectors;

fn main() {
  println!("vectors call:");
  vectors::vector_creation();
  vectors::vector_access();
  vectors::vector_iteration();
  vectors::vector_enum();
  println!("\nstrings call:");
  strings::string_creation();
  strings::string_access();
  println!("\nhash_maps call:");
  hash_maps::map_creation();
  hash_maps::map_access();
  hash_maps::map_update();
  println!("\npractices call:");
  practices::practice1();
  practices::practice2();
  println!("\nEnd of main function");
}
