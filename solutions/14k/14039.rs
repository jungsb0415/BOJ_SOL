use std::{io, vec};

fn main() {
    
    let mut vv: Vec<Vec<i32>> = vec![];

    let mut input: String = String::new();

    for _i in 0..4 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
        vv.push(v);
    }    

    let mut sums: Vec<i32> = vec![];

    for i in 0..4 {
        let mut sum = 0;
        for j in 0..4 {
            sum += vv[i][j];
        }
        sums.push(sum);
    }

    for i in 0..4 {
        let mut sum = 0;
        for j in 0..4 {
            sum += vv[j][i];
        }
        sums.push(sum);
    }

    let mut p = true;

    for i in 0..7 {
        if sums[i] != sums[i+1] {
            p = false;
        }
    }

    if p {
        print!("magic\n");
    }
    else {
        print!("not magic\n");
    }
    
}
