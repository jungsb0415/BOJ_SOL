use std::io;

fn main() {
    
    let mut input: String = String::new();

    let mut field: Vec<Vec<char>> = vec![];

    for _ in 0..8 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<char> = input.trim().chars().collect();

        field.push(line);
    }

    let mut cnt = 0;

    for i in 0..8 {
        for j in 0..8 {
            if field[i][j] == '.' {
                let mut p = true;

                for k in 0..8 {
                    if field[i][k] == 'R' {
                        p = false;
                        break;
                    }
                    if field[k][j] == 'R' {
                        p = false;
                        break;
                    }
                }

                if p {
                    cnt += 1;
                }
            }
        }
    }

    print!("{}", cnt);


}
