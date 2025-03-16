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
    
    let v = get_ints();
    let mut a = v[0];
    let mut b = v[1];
    
    let mut ans = 0;

    let mut bit = 1;

    while a != 0 || b != 0 {
        let k = b%2;
        ans += k * bit;
        bit *= 2;
        b /= 2;
        let k = a%2;
        ans += k * bit;
        bit *= 2;
        a /= 2;
    }

    print!("{}", ans);
    


}
