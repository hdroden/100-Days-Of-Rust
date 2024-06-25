fn main() {
    let grill: [String; 5] = ["--xo--x--ox--".to_string(),
        "--xx--x--xx--".to_string(),
        "--oo--o--oo--".to_string(),
        "--xx--x--ox--".to_string(),
        "--xx--x--ox--".to_string()];
   let mut veg = 0;
   let mut meat = 0;

    for skewer in grill {
        if skewer.find('x') != None {
            meat = meat + 1;         
        } else {
            veg = veg + 1;
        }
    }

    println!("[{meat}, {veg}]");
}

