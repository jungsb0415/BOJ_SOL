use std::io;

fn main() {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let v: Vec<char> = input.trim().chars().collect();

        let l = v.len();

        if l == 1 {
            print!("{}\n", v[0]);
            continue;
        }

        for i in 0..l {
            print!("{}", v[i]);
        }
        print!("\n");

        for i in 1..l-1 {
            print!("{}", v[i]);
            for j in 1..l-1 {
                print!(" ");
            }
            print!("{}\n", v[l-i-1]);
        }

        for i in 0..l {
            print!("{}", v[l-1-i]);
        }
        print!("\n");

        
    }

    

        

    
    
}
