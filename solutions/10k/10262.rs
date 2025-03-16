use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    let a1 = v[0];
    let b1 = v[1];
    let a2 = v[2];
    let b2 = v[3];

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace()
    .map(|x| x.parse().unwrap()).collect();

    let a3 = v[0];
    let b3 = v[1];
    let a4 = v[2];
    let b4 = v[3];

    let mut p = 0;
    let mut q = 0;

    for i in a1..b1 {
        for ii in a2..b2 {
            for iii in a3..b3 {
                for iv in a4..b4 {
                    if i+ii > iii+iv {p += 1}
                    else if i+ii < iii+iv {q += 1}
                }
            }
        }
    }

    if p > q {print!("Gunnar")}
    else if p < q {print!("Emma")}
    else {print!("Tie")}
    

}
