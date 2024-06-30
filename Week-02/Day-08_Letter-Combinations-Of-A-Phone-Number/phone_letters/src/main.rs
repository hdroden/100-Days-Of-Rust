use std::io;

fn main() {}

struct Letter {
    let1: char,
    let2: char,
    let3: char,
    let4: Option<char>,
}

struct Tree {
    root: Letter,
    out_str: String,
    next: Option<Tree>,
}

impl Tree {
    fn new_str(&mut self, input: String) {
        self.out_str = input;
    }
}

fn map_numbers(input: String) -> Letter {
    if input == "2" {
        return Letter {
            let1: 'a',
            let2: 'b',
            let3: 'c',
            let4: None,
        };
    } else if input == "3" {
        return Letter {
            let1: 'd',
            let2: 'e',
            let3: 'f',
            let4: None,
        };
    } else if input == "4" {
        return Letter {
            let1: 'g',
            let2: 'h',
            let3: 'i',
            let4: None,
        };
    } else if input == "5" {
        return Letter {
            let1: 'j',
            let2: 'k',
            let3: 'l',
            let4: None,
        };
    } else if input == "6" {
        return Letter {
            let1: 'm',
            let2: 'n',
            let3: 'o',
            let4: None,
        };
    } else if input == "7" {
        return Letter {
            let1: 'q',
            let2: 'r',
            let3: 's',
            let4: None,
        };
    } else if input == "8" {
        return Letter {
            let1: 't',
            let2: 'u',
            let3: 'v',
            let4: None,
        };
    } else if input == "9" {
        return Letter {
            let1: 'w',
            let2: 'x',
            let3: 'y',
            let4: Some('z'),
        };
    } else {
        let mut usr_input = String::new();

        println!("Please enter a valid number 2-9");
        io::stdin().read_line(&mut usr_input).unwrap();
        return map_numbers(usr_input);
    }
}

fn build_tree(input: String) -> Tree {
    if input.len() == 1 {
        return Tree {
            root: map_numbers(String::from(input)),
            out_str: String::new(),
            next: None,
        };
    } else {
        let mut chars = input.chars();
        let first_let = chars.next().unwrap();

        return Tree {
            root: map_numbers(first_let.to_string()),
            out_str: String::new(),
            next: Some(build_tree(String::from(chars.as_str()))),
        };
    }
}
//REMEBER TO PASS IN LENGHT -1 to avoid enter as the last character
