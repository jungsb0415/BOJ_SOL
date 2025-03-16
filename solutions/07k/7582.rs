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
    
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut input = input.trim().split_whitespace();

        let name = input.next().unwrap();
        let max: i64 = input.next().unwrap().parse().unwrap();

        if name == "#" {break;}

        let mut cur = get_int();
        let n = get_int() as usize;

        for i in 0..n {
            let v = get_ints();
            cur += v[1] - v[0];
            if cur > max {
                cur = max;
            }
        }

        print!("{} {}\n", name, cur);
    }

    

    


}
