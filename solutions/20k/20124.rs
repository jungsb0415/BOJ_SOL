use std::io::{self, BufRead};

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut v: Vec<(i32, String)> = vec![];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        
        let text: String = parts.next().unwrap().to_string();
        let number: i32 = parts.next().unwrap().parse().unwrap();

        v.push((number, text));
    }

    v.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1))); 
    let l = v.len();

    print!("{}", v[l-1].1);


}
