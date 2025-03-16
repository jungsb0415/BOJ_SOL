use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<char> = input.trim().chars().collect();

    let mut num = false;
    let mut small = false;
    let mut big = false;
    let mut what = false;

    for ch in &input {
        if '0' <= *ch && *ch <= '9' {num = true}
        else if 'a' <= *ch && *ch <= 'z' {small = true}
        else if 'A' <= *ch && *ch <= 'Z' {big = true}
        else {what = true}
    }

    let mut cnt = 4;
    if num {cnt -= 1}
    if small {cnt -= 1}
    if big {cnt -= 1}
    if what {cnt -= 1}
    let dnt = 6 - input.len() as i32;

    if cnt > dnt {print!("{}", cnt)}
    else if dnt >= 0 {print!("{}", dnt)}
    else {print!("0")}
   
    
}
