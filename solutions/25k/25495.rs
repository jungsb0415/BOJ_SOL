use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    let mut cur = 2;
    let mut sum = 2;
    let mut past = v[0];

    for i in 1..n {
        if past == v[i] {
            cur *= 2;
            sum += cur;
        }
        else {
            cur = 2;
            sum += cur;
        }

        if sum >= 100 {
            sum = 0;
            cur = 0;
            past = -1;
        }
        else {
            past = v[i];
        }
    }
    
    print!("{}", sum);

}
