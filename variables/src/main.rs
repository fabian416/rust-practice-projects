fn main() {
  let s = String::from("Hello");

  let (s2, len) = calculate_length(s);
  
  println!("The legth of {s2} is: {len}");

} 

fn calculate_length(s: String) -> (String, usize) { 
  let length = s.len();
  
  (s, length)
}
