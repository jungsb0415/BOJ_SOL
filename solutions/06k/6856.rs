use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    let mut cnt = 0;
    for i in 1..10 {
        if i <= a && (10 - i) <= b {
            cnt += 1;
        }
    }

    if cnt == 1 {
        print!("There is 1 way to get the sum 10.");
    }
    else {
        print!("There are {} ways to get the sum 10.", cnt);
    }
}
