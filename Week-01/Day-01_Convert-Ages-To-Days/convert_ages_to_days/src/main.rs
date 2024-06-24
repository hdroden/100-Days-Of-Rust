use std::io;

fn main() {
   println!("Enter the number of years you want to convert: ");

   let mut age = String::new(); 
   io::stdin().read_line(&mut age).unwrap();

   let age: i32 = age.trim().parse().unwrap();

   println!("The age in days is {}", age * 365 + (age / 4));

}
