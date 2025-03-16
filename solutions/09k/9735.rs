use std::io;
// use std::collections::HashSet;

static LIMIT: f64 = 1_000_000.0;
static INTLIMIT: i128 = 1_000_000;

fn main() {
    let mut input : String = String::new();
    let _ = io::stdin().read_line(&mut input);     
    let n : i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        let _ = io::stdin().read_line(&mut input);

        let coef : Vec<i128> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        let a = coef[0] as i128;
        let b = coef[1] as i128;
        let c = coef[2] as i128;
        let d = coef[3] as i128;

        let mut solint = INTLIMIT+1;

        let pan = b*b-3*a*c;

        for i in -INTLIMIT..=INTLIMIT {
            if ((a*i+b)*i+c)*i+d == 0{
                solint = i;
                break;
            }
        }

        let mut ans = [solint as f64].to_vec();

        let a = a;
        let b = b + solint*a;
        let c = c + solint*b;

        // print!("{} {} {}\n",a,b,c);

        let d = b*b-4*a*c;

        if d < 0 {
    
        }
        else if d == 0 {
            if b%(2*a) == 0 {
                let sol = -b/(2*a);

                if solint != sol {
                    ans.push(sol as f64);
                }
            }
            else {
                let sol = (-b) as f64 / (2*a) as f64;

                ans.push(sol);
            }
        }
        else if d > 0 {
            let x1 = (-b as f64 - (d as f64).sqrt()) / (2.0 * a as f64);
            let x2 = (-b as f64 + (d as f64).sqrt()) / (2.0 * a as f64);
            ans.push(x1);
            ans.push(x2);
        }

        let l = ans.len();
        for _ in 0..l-1 {
            for j in 0..l-1 {
                if ans[j] > ans[j+1] {
                    let tmp = ans[j];
                    ans[j] = ans[j+1];
                    ans[j+1] = tmp;
                }
            }
        }

        let mut bns = [ans[0]].to_vec();

        if l > 1 && ans[0] != ans[1] {
            bns.push(ans[1]);
        }
        if l > 2 && ans[1] != ans[2] {
            bns.push(ans[2]);
        }

        for num in &bns {
            print!("{} ", num);
        }
        print!("\n");
    }
}