use std::io;

fn main() {
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse().unwrap();

    for i in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input.trim();
        let v: Vec<char> = line.chars().collect();

        let mut idx = 0;
        while v[idx] == '!' {idx += 1}

        let l = line.len();
        
        let a = idx;
        let b = l - idx - 1;

        let mut n = 0;
        if v[idx] == '1' {n = 1}

        if n == 0 && b > 0 {n = 1}

        if a%2 == 1 {
            n = 1 - n;
        }

        print!("{}\n", n);


    }
    

}
