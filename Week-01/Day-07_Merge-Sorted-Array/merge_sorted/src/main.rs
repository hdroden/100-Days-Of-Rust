fn main() {
    let mut num1: [i32; 6] = [1, 2, 3, 0, 0, 0];
    let num2: [i32; 3] = [2, 5, 6];
    let m = 3;
    let n = 3;

    let mut m_mut: i32 = m;
    let mut n_mut: i32 = n;

    for i in (0..num1.len()).rev() {
        if m_mut > 0 && n_mut == 0 {
            let m_pos: usize = usize::try_from(m_mut - 1).unwrap();
            num1[i] = num1[m_pos];
            m_mut = m_mut - 1;
            continue;
        } else if m_mut == 0 && n_mut > 0 {
            let n_pos: usize = usize::try_from(n_mut - 1).unwrap();
            num1[i] = num2[n_pos];
            n_mut = n_mut - 1;
            continue;
        } else if m_mut == 0 && n_mut == 0 {
            break;
        }
        let m_pos: usize = usize::try_from(m_mut - 1).unwrap();
        let n_pos: usize = usize::try_from(n_mut - 1).unwrap();

        if m_mut > 0 && n_mut > 0 {
            if num1[m_pos] >= num2[n_pos] {
                num1[i] = num1[m_pos];
                m_mut = m_mut - 1;
            } else {
                num1[i] = num2[n_pos];
                n_mut = n_mut - 1;
            }
        }
    }

    for i in num1 {
        println!("The final result is: {i}");
    }
}
