use std::io;

fn main() {
    let mut input_string = String::new();
    let nemo = String::from("nemo"); 

    println!("Please enter a string to find nemo in");
    io::stdin().read_line(&mut input_string).unwrap();
    let split_version = input_string.split(' ');

    let mut found = false;
    let mut i = 0;
    for text in split_version {

       if text.trim() == nemo {
            println!("You found nemo at {i}");
            found = true;
       }
       if found == true {
           break;
       }
       i = i +1;
    }
    if found == false {
        println!("Nemo was not found");
    }
}
