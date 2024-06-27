use std::io;

fn main() {
    println!("Output #1 should be 1: {}", sock_pairs("AA"));
    println!("Output #2 should be 2: {}", sock_pairs("ABABC"));
    println!("Output #3 should be 4: {}", sock_pairs("CABBACCC"));
    println!("Output #4 should be 0: {}", sock_pairs(""));

    let mut user_input = String::new();
    println!("Please enter the string to Check: ");
    io::stdin().read_line(&mut user_input).unwrap();

    println!("Output for user input is: {}", 
        sock_pairs(&user_input[0..user_input.len() - 1]));
}

fn sock_pairs(total_socks: &str) -> i32 {
    let mut total_pairs = 0;
    let mut singles: [bool; 26] = [false; 26]; 
    
    for x in total_socks.chars() {

        let indicies = char_mapping(x);
        let indicies = usize::try_from(indicies).unwrap();

        if singles[indicies] == true {
            singles[indicies] = false;
            total_pairs = total_pairs + 1;
        } else {
            singles[indicies] = true;
        }
    }
    return total_pairs;
}

fn char_mapping(letter: char) -> u32 {
    let num = letter as u32 - 'A' as u32;
    return num
}
