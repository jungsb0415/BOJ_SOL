use std::io;

fn max_under_k(alpha: &Vec<i32>, t: i32) -> i32 {  
    let mut ret = -1;
    
    for n in alpha {
        if ret < *n && *n < t {
            ret = *n;
        }
    }

    ret
}

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input: Vec<char> = input.trim().chars().collect();

    let mut alpha: Vec<i32> = [0; 26].to_vec();

    for ch in &input {
        alpha[((*ch) as u8 - 'a' as u8) as usize] += 1;
    }
 
    let mut t = input.len() as i32 + 1;
    loop {
        t = max_under_k(&alpha, t);
        if t == -1 {break;}

        for i in 0..26 {
            if alpha[i] == t {
                for _ in 0..t {
                    print!("{}", (i as u8 + 'a' as u8) as char);
                }
            }
        }
    }


}
