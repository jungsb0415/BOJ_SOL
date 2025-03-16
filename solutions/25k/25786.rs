use std::io;

fn main() {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut b: i32 = input.trim().parse().unwrap();

    let mut ans: Vec<i32> = vec![];

    while a != 0 || b != 0 {
        let p = a%10;
        let q = b%10;

        if p < 3 && q < 3 || p > 6 && q > 6 {
            ans.push(0);
        }
        else {
            ans.push(9);
        }
        a /= 10;
        b /= 10;
    }

    let l = ans.len();
    if l == 0 {ans.push(0);}
    let l = ans.len();

    for i in 0..l {
        print!("{}", ans[l-1-i]);
    }

        
}
