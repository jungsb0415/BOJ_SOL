use std::io;

fn main() {
    
    let mut input: String = String::new();

    let mut past_a = 0.0;
    let mut past_b = 0.0;
    let mut km = 0.0;
    let mut puel = 0.0;
    let mut ans: f64 = 0.0;

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let ns: Vec<f64> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        let a = ns[0];
        let b = ns[1];

        if ns[0] == -1.0 {break;}
        if ns[0] == 0.0 {
            ans = km / puel * past_b;
            print!("{}\n", ans.round());
            past_a = 0.0;
            past_b = 0.0;
            km = 0.0;
            puel = 0.0;
        }

        if b < past_b {
            km += past_a - a;
            puel += b - past_b;
        }

        past_a = a;
        past_b = b;


    }



}
