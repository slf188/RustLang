fn main(){
  /* example 1
  stack: x | i32 | 5
  */
  let x: i32 = 5;
  
  /* example 2
  stack: vec1 | pointer -> heap: 1, 2, 3, 4
  */
  let mut vec1 = vec![1, 2, 3];
  vec1.push(4);
  println!("{}", vec1);
  drop(vec1);
  
  // example 3
  let mut say = String::from("Do");
  say.push_str("g");
  let say2 = say;
  drop(say);

  // example 4
  let say3 = String::from("Cat");
  print_out(&say3);
  println!("The original string var's value is {}", say3);
}

fn print_out(say: &String){
  println!("Here we pass the var as a reference {}", say);
}
