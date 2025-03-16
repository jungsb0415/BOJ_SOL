use std::{io, vec};

fn get_ints() -> Vec<i64> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i64> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();
    v
}

fn get_int() -> i64 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: i64 = input.trim().parse().unwrap();
    v
}

fn main() {
    
    let t = get_int();

    for _ in 0..t {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let v: Vec<char> = input.trim().chars().collect();

        let n = v.len();
        let mut idx = n-1;

        while idx > 0 && v[idx-1] > v[idx] {
            idx -= 1;
        }

        print!("The longest decreasing suffix of {} is ", input.trim());
        for i in idx..n {
            print!("{}", v[i]);
        }
        print!("\n");
    }    


}
