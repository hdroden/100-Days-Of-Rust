fn main() {

    let input1: [i32;4] = [1,2,3,4];
    let input2: [i32;4] = [1,1,1,1];
    let input3: [i32;5] = [10,11,12,9,10];
    let input5: [i32;10] = [10,11,12,9,10,23,56,78,90,21];
    let input4: [i32;1] = [4];
    let res = progress_days(&input1);
    let res2 = progress_days(&input2);
    let res3 = progress_days(&input3);
    println!("{}", res);
    println!("{}", res2);
    println!("{}", res3);
    println!("{}", progress_days(&input4));
    println!("{}", progress_days(&input5));
}

fn progress_days(days: &[i32]) -> i32 {
    let mut counter = 0;
    if days.len() == 1 || days.len() == 0 {
        println!("The Array does not have a lot of numbers");
    }
    for i in 0..days.len()-1 {
        if days[i] < days[i+1] {
            counter = counter + 1;
        }
    }
    return counter;
}
