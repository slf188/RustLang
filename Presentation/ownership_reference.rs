fn main(){
  let mut say = String::from("Do");
  say.push_str("g");
  print_out(&say);
  println!("The original string var's value is {}", say);
  let say2 = say;
  drop(say);
  println!("The copy would therefore contain {}", say2);
}

fn print_out(say: &String){
  println!("Here we pass the var as a reference {}", say);
}
