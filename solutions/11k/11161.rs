use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _i in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut cur = 0;
        let mut min = 0;
        
        for _j in 0..n {
            input.clear();

            io::stdin().read_line(&mut input).unwrap();
            let line: Vec<i32> = input.split_whitespace()
            .map(|x| x.parse().unwrap()).collect();

            let a = line[0];
            let b = line[1];

            cur += a-b;

            if min > cur {min = cur};
        }
        print!("{}\n", -min);
    }
}
