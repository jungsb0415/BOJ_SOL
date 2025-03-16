use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut vv: Vec<Vec<i32>> = vec![];

    let mut max = -1;
    let mut idx = 0;

    for i in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let v: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        for j in 0..n {
            if max < v[j] {
                max = v[j];
                idx = i;
            }
        }

        vv.push(v);
    }

    let ans: Vec<&str> = 
    ["PROBRAIN", "GROW", "ARGOS", "ADMIN", "ANT", "MOTION", "SPG", "COMON", "ALMIGHTY"].to_vec();

    print!("{}", ans[idx as usize]);

}
