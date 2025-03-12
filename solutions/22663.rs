use std::{io, vec};
// use std::collections::HashSet;

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    exit: bool,
    door: [(bool, bool); 4]
    // (벽이 없어서 통과가능한지, 벽을 방문했는지)
}

impl Node {
    fn new() -> Node {
        Node {
            exit: false,  
            door: [(true, false); 4]
        }
    }
}


fn init(field: &mut Vec<Vec<Node>>) {
    let w = field.len();
    let h = field[0].len();

    for i in 0..w {
        field[i][0].door[2].0 = false;
        field[i][h-1].door[0].0 = false;
    }

    for i in 0..h {
        field[0][i].door[3].0 = false;
        field[w-1][i].door[1].0 = false;
    }
}

fn wall(field: &mut Vec<Vec<Node>>, x1:usize, y1:usize, x2:usize, y2:usize) {
    if x1 == x2 {
        for i in y1..y2 {
            field[x1-1][i].door[1].0 = false;
            field[x1][i].door[3].0 = false;
        }
    }
    if y1 == y2 {
        for i in x1..x2 {
            field[i][y1-1].door[0].0 = false;
            field[i][y1].door[2].0 = false;
        }
    }
}

fn print(field: &Vec<Vec<Node>>, cur : (usize, usize), cur_see : usize) {
    let w = field.len();
    let h = field[0].len();

    for i in 0..h {
        for j in 0..w {
            print!("#");
            let node = field[j][h-1-i].clone();
            if node.door[0].0 == false {
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        print!("#\n");

        for j in 0..w {
            let node = field[j][h-1-i].clone();
            if node.door[3].0 == false {
                print!("#");
            }
            else {
                print!(" ");
            }
            if node.exit {
                print!("E");
                continue;
            }
            else if cur == (j, h-1-i) {
                print!("{}", cur_see);
                continue;
            }
            else {print!(" ");}
        }
        print!("#\n");
    }
    for j in 0..w {
        print!("#");
        let node = field[j][0].clone();
        if node.door[2].0 == false {
            print!("#");
        }
        else {
            print!(" ");
        }
    }
    print!("#\n");

}

fn main() {

    let mut input = String::new();
    loop {
        input.clear();
        let _ = io::stdin().read_line(&mut input);     

        let nums : Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        let w = nums[0] as usize;
        let h = nums[1] as usize;
        let n = nums[2] as usize;

        if w+h+n == 0 {break;}

        let mut field : Vec<Vec<Node>> = vec![vec![Node::new(); h]; w];
        
        init(&mut field);

        for _ in 0..n {
            input.clear();
            let _ = io::stdin().read_line(&mut input);
            let nums : Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
            let x1 = nums[0];
            let y1 = nums[1];
            let x2 = nums[2];
            let y2 = nums[3];
            
            if x1 > x2 {
                wall(&mut field, x2, y1, x1, y2);
            }
            else if y1 > y2 {
                wall(&mut field, x1, y2, x2, y1);
            }
            else {
                wall(&mut field, x1, y1, x2, y2);
            }
        }

        input.clear();
        let _ = io::stdin().read_line(&mut input);
        let nums : Vec<usize> = input.split_whitespace()
        .map(|x| x.parse().unwrap()).collect();

        let mut x1 = nums[0];
        let mut y1 = nums[1];
        let mut x2 = nums[2];
        let mut y2 = nums[3];
        let x3 = nums[4];
        let y3 = nums[5];
        field[x3][y3].exit = true;

        let mut cur = (1, 1);
        let mut cur_see = 0;

        if x1 > x2 {(x1, x2) = (x2, x1);}
        if y1 > y2 {(y1, y2) = (y2, y1);}

        if x1 == x2 {
            if x1 == 0 {
                cur_see = 1;
                cur = (0, y1);
            }
            else if x1 == w {
                cur_see = 3;
                cur = (x1-1, y1);
            }
        }
        if y1 == y2 {
            if y1 == 0 {
                cur_see = 0;
                cur = (x1, 0);
            }
            else if y1 == h {
                cur_see = 2;
                cur = (x1, y1-1);
            }
        }

        let mut x = cur.0;
        let mut y = cur.1;

        let mut cnt = 1;
        let mut p = true;

        while field[x][y].exit == false {
            x = cur.0;
            y = cur.1;

            // print
            // print(&field, cur, cur_see);
 
            // visit
            if field[x][y].door[(cur_see+3)%4].1 {
                p = false;
                break;
            }
            field[x][y].door[(cur_see+3)%4].1 = true;

            // move

            let can_go = field[x][y].door[cur_see].0;
            let exist_left_wall = !field[x][y].door[(cur_see + 3) % 4].0;

            if can_go && exist_left_wall {
                if cur_see == 0 {y += 1;}
                if cur_see == 1 {x += 1;}
                if cur_see == 2 {y -= 1;}
                if cur_see == 3 {x -= 1;}
                cnt += 1;
            }
            else if exist_left_wall {
                cur_see = (cur_see + 1) % 4;
            }
            else {
                cur_see = (cur_see + 3) % 4;
                if cur_see == 0 {y += 1;}
                if cur_see == 1 {x += 1;}
                if cur_see == 2 {y -= 1;}
                if cur_see == 3 {x -= 1;}
                cnt += 1;
            }

            cur = (x, y);
        }

        if p {print!("{}\n", cnt);}
        else {print!("Impossible\n");}

        
       
        


    }
}