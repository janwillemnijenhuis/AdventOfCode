use std::fs;

fn main() {
    let mut contents = fs::read_to_string("input.txt")
        .expect("Could not read file.");
    let mut countlines = count_lines(&contents);
    let mut numboards: usize = ((countlines - 1) / 6) as usize;
    let mut drawnnumber: &str = Default::default();
    let mut boards = vec![vec![vec![0i32; 5]; 5]; numboards];
    let mut binboards = vec![vec![vec![0i32; 5]; 5]; numboards];
    let mut boardcount: usize = 0;

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            drawnnumber = line;
            continue;
        } else if i == 1 || (((i - 1) % 6 == 0) && i > 2){
            continue;
        } else {
            if ((i - 1) % 6 == 1) && i > 3  {
                boardcount += 1;
            }
            for (j, item) in line.split_whitespace().enumerate() {
                boards[boardcount][(i - 2) % 6][j] = item.parse::<i32>().unwrap();
            }
        }
    }


    let mut winning_board: Vec<Vec<i32>>;
    let mut winning_binboard: Vec<Vec<i32>>;
    let mut result = 0;
    let mut winner = false;
    'outer: for (i, item) in drawnnumber.split(',').enumerate() {
        let mut number = item.parse::<i32>().unwrap();
        'inner: for j in 0..numboards {
            (boards[j], binboards[j]) = check_num(boards[j], binboards[j], number);
            (winner, result) = check_won(boards[j], binboards[j]);
            if winner {
                winning_board = boards[j];
                winning_binboard = binboards[j];
                break 'outer;
            }

        }
    }

    //println!("{:?}", boards);
}

fn count_lines(s: &String) -> i32 {
    let mut count: i32 = 0;
    for _ in s.lines() {
        count += 1;
    }
    count
}

fn check_num(board: Vec<Vec<i32>>, binboard: Vec<Vec<i32>>, num: i32) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    for i in 0..5 {
        for j in 0..5 {
            if binboard[i][j] == 1 {
                continue;
            } else if board[i][j] == num {
                binboard[i][j] == 1;
            }
        }
    }
    return (board, binboard);
}

fn check_won(board: Vec<Vec<i32>>, binboard: Vec<Vec<i32>>) -> (bool, i32) {
    let mut won = false;
    let mut result = 0;
    for i in 0..5 {
        let mut rowsum = 0;
        let mut colsum = 0;
        for j in 0..5 {
            rowsum += binboard[i][j];
            colsum += binboard[j][i];
        }
        if rowsum == 5 {
            for k in 0..5 {
                result += board[i][k];
                return (true, result);
            }
            for l in 0..5 {
                result += board[k][i];
                return (true, result);
            }
        }
    }
    return (false, result);
}