use std::io;

fn f(n: i32) -> bool {

    let mut v: Vec<i32> = [].to_vec();
    let mut n = n;

    while n != 0 {
        v.push(n%10);
        n /= 10;
    }

    let l = v.len();
    for i in 0..l/2 {
        let a = v[i];
        let b = v[l-1-i];

        if !((a == 0 && b == 0) ||
        (a == 1 && b == 1) ||
        (a == 6 && b == 9) ||
        (a == 8 && b == 8) ||
        (a == 9 && b == 6)) {
            return false;
        }
    }

    if l%2 == 1 {
        let l = l/2;
        if !(v[l] == 0 || v[l] == 1 || v[l] == 8) {return false;}
    }

    true
}

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: i32 = input.trim().parse().unwrap();

    let mut cnt = 0;
    for i in n..=m {
        if f(i) {
            cnt += 1;
        }
    }

    print!("{}", cnt);


}
