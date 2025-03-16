use std::io;

fn main() {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let v: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    for i in 0..n {
        let mut cnt = 1;
        for j in 0..n {
            if v[i] > v[j] {cnt += 1}
        }
        print!("{}\n", cnt);
    }
    
    
}
