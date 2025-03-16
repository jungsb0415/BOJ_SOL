use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _i in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let a = n/100;
        let b = n%100;

        let c = a*a + b*b;
        if c%7 == 1 {
            print!("YES\n");
        }
        else {
            print!("NO\n");
        }

    }
}
