#![feature(isqrt)]
fn main() {
    println!("The output should be 13: {}", next_prime(12));
    //println!("The output should be 29: {}", next_prime(24));
    //println!("The output should be 11: {}", next_prime(11));
}

fn next_prime(input: i32) -> i32 {
    let mut prime_list: Vec<i32> = vec![2, 3];
    let mut current_itr: i32 = 3;
    let input_sqrt: i32 = input.isqrt();
    //while current_itr < input_sqrt{
    while current_itr < 30 {
        let mut is_prime: bool = true;
        let itr_sqrt: i32 = current_itr.isqrt();

        //Calculate the upper bound
        //Returns the index of where the upperboundry is
        let mut upbound: i32 = 0;
        for x in prime_list {
            if itr_sqrt < x {
                break;
            } else {
                upbound = upbound + 1;
            }
        }

        //iterates until upper boundry is reached
        for x in 0..=upbound {
            if current_itr % prime_list[x] == 0 {
                is_prime = false;
                break;
            }
        }
        if !is_prime {
            println!("{current_itr} is not prime.");
            continue;
        } else if current_itr < input_sqrt {
            prime_list.push(current_itr);
            println!("{current_itr} is prime but not the next prime");
        } else if current_itr >= input {
            break;
        }

        current_itr = current_itr + 1;
    }
    // while kkkj
    //
    return current_itr;
}
