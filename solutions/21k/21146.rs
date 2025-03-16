use std::io;

fn main() {
    
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let line: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    let n = line[0];
    let k = line[1];

    let mut sum = 0;

    for _i in 0..k {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let t: i32 = input.trim().parse().unwrap();
        sum += t;
    }

    print!("{} {}", (sum - 3 * (n-k))as f64 / n as f64, (sum + 3 * (n-k)) as f64 / n as f64);
}
