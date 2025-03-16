use std::{io, vec};

fn f(ch: char) -> bool {
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
        return true;
    }
    false
}
fn main() {
    

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let v: Vec<char> = input.trim().chars().collect();

    let l = v.len();

    let mut p = true;

    for i in 1..l {
        if f(v[i-1]) == f(v[i]) {
            p = false;
        }
    }

    if p {
        print!("1")
    }
    else {
        print!("0")
    }
    
    
}
