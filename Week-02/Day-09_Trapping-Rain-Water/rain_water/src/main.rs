fn main() {
    //let height: [i32; 12] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let height: [i32; 6] = [4, 2, 0, 3, 2, 5];

    let mut curr_pos: i32 = 0;
    let mut curr_height: i32 = height[curr_pos as usize];
    let mut next_pos: i32 = 0;
    let mut next_height: i32 = height[curr_pos as usize];
    let mut score: i32 = 0;
    let mut curr_biggest_height = 0;

    while next_pos != height.len().try_into().unwrap() {
        while next_height <= curr_height {
            //while next_height < height[(next_pos - 1) as usize] && next_pos - curr_pos > 2 {
            next_pos = next_pos + 1;
            if next_pos == height.len().try_into().unwrap() {
                break;
            }
            next_height = height[next_pos as usize];
        }
        println!("Next pos is: {next_pos} and next height is {next_height} curr_pos is {curr_pos} and score is {score}");
        if next_pos == height.len().try_into().unwrap() && next_height < curr_biggest_height {
            curr_pos = curr_pos + 1;
            curr_height = height[curr_pos as usize];
            // next_pos = curr_pos;
            //next_height = curr_height;
            curr_biggest_height = curr_height;
            while height.len() as i32 - curr_pos > 1 && curr_biggest_height > 0 {
                println!("current_ pos is {curr_pos} ");
                if curr_biggest_height - curr_height > 0 {
                    score = score + curr_biggest_height - curr_height;
                }
                curr_pos = curr_pos + 1;
                if curr_pos == height.len().try_into().unwrap() {
                    break;
                }
                curr_height = height[curr_pos as usize];
            }
            break;
        }

        curr_biggest_height = curr_height;
        while next_pos - curr_pos > 0 {
            let temp = (curr_pos + 1) as usize;
            if curr_biggest_height < curr_height {
                curr_biggest_height = curr_height;
            }

            if curr_biggest_height > height[temp] {
                score = score + curr_biggest_height - height[temp];
            }
            curr_pos = curr_pos + 1;
            curr_height = height[temp];
        }
        if curr_pos + 1 == height.len().try_into().unwrap() {
            break;
        }
    }

    println!("{score}");
}
