use std::io;

fn main() {
    
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let line: Vec<char> = input.trim().chars().collect();

    let mut even = true;
    let mut odd = true;

    let mut alpha: Vec<i32> = [0; 26].to_vec();

    for ch in &line {
        alpha[(*ch as u8 - 'a' as u8) as usize] += 1;
    }

    for i in 0..26 {
        if alpha[i] > 0 {
            if alpha[i] % 2 == 0 {
                odd = false;
            }
            else {
                even = false;
            }
        }
    }

    if even {print!("0")}
    else if odd {print!("1")}
    else {print!("2")}


}
