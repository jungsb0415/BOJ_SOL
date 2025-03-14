use std::io;

fn f(n: i32) -> [i32; 9] {

    let mut ret: [i32; 9] = [0; 9];
    let mut n = n;

    while n != 0 {
        let k = n%10;
        if k > 0 {
            ret[k as usize - 1] += 1;
        }
        n /= 10;
    }

    ret
}

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for i in 0..n {
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut m = n + 1;
        
        while f(m) != f(n) {
            m += 1;
        }

        print!("Case #{}: {}\n", i+1, m);
    }


}
