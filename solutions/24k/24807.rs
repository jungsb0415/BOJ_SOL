use std::io;

fn main() {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let v: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    let a = v[0];
    let b = v[1];
    let c = v[2];
    let sum = v[3];

    let mut cnt = 0;

    for i in 0..1000 {
        if a * i > sum {break;}

        for j in 0..1000 {
            if a * i + b * j > sum {break;}

            let tmp = sum - a*i - b*j;

            if tmp % c == 0 {
                cnt += 1;
                print!("{} {} {}\n", i, j, tmp/c);
            }
        }
    }

    if cnt == 0 {print!("impossible");}
    
}
