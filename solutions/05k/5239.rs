use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    
    for _i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let ns: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        let k = ns[0];

        let mut xs: Vec<i32> = vec![];
        let mut ys: Vec<i32> = vec![];

        for j in 0..k {
            xs.push(ns[(2*j + 1) as usize]);
            ys.push(ns[(2*j + 2) as usize]);
        }

        xs.sort();
        ys.sort();

        let mut p = true;
        for i in 1..k as usize{
            if xs[i-1] == xs[i] || ys[i-1] == ys[i] {
                p = false;
            }
        }

        if p {print!("SAFE\n");}
        else {print!("NOT SAFE\n");}

    }



}
