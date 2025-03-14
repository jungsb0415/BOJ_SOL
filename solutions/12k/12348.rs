use std::io;
// use std::collections::HashSet;

fn max(a: i64, b: i64) -> i64 {
    if a > b {return a;} b
}

fn f(a: i64) -> i64 {
    let mut ret = a;
    let mut a = a;
    while a != 0 {
        ret += a%10;
        a /= 10;
    }
    ret
}

fn main() {
    let mut input : String = String::new();
    let _ = io::stdin().read_line(&mut input);     
    let n : i64 = input.trim().parse().unwrap();

    let k = max(1, n-200);
    let mut p = false;
    let mut ans = -1;
    for i in k..n {
        if f(i) == n {
            p = true;
            ans = i;
            break;
        }
    }
    if !p {ans = 0;}
    print!("{}", ans);
    
}