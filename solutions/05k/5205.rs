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
    
    let t = get_int();

    for _i in 0..t {
        let n = get_int() as usize; 

        let mut tuvec: Vec<(i64, i64, i64)> = [].to_vec();

        for _j in 0..n {
            let v = get_ints();
            tuvec.push((v[0], v[1], v[2]));
        }

        let mut max = 0i64;
        let mut ans: Vec<(usize, usize)> = vec![];
        for i in 0..n {
            for j in i..n {
                let dx = tuvec[j].0 - tuvec[i].0;
                let dy = tuvec[j].1 - tuvec[i].1;
                let dz = tuvec[j].2 - tuvec[i].2;

                let l = dx*dx + dy*dy + dz*dz;

                if l > max {
                    max = l;
                    ans.clear();

                    ans.push((i, j));
                }
                else if l == max {
                    ans.push((i, j));
                }
            }
        }

        ans.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))); 
    
        print!("Data Set {}:\n", _i+1);
        for (a, b) in ans {
            print!("{} {}\n", a+1, b+1);
        }
    }

    


}
