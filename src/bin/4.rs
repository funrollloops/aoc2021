use std::io;
// use std::io::BufRead;

type Board = [[i32; 5]; 5];

fn read_draws(stdin: &io::Stdin) -> io::Result<Vec<i32>> {
    let mut draws: Vec<i32> = Vec::new();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    for num in buffer.trim().split(",") {
        draws.push(num.parse().unwrap());
    }
    Ok(draws)
}

fn read_board(stdin: &io::Stdin) -> io::Result<Option<Board>> {
    let mut buffer = String::new();
    let mut board: Board = [[-1; 5]; 5];
    for row in 0..5 {
        while buffer.trim().is_empty() {
            if stdin.read_line(&mut buffer)? == 0 {
                return Ok(None)
            }
        }
        let mut col = 0;
        for num in buffer.trim().split_whitespace() {
            let num = num.parse().unwrap();
            board[row][col] = num;
            col += 1;
        }
        buffer.clear();
    }
    Ok(Some(board))
}

fn check_won(board: &Board) -> bool {
    for i in 0..5 {
        let mut col = 0;
        let mut row = 0;
        for j in 0..5 {
            if board[i][j] < 0 { row += 1 }
            if board[j][i] < 0 { col += 1 }
        }
        if col == 5 || row == 5 {
            return true
        }
    }
    return false
}

fn check_board(draws: &Vec<i32>, mut board: Board) -> (usize, i32) {
    for (turn, draw) in draws.iter().enumerate() {
        let mut score = 0;
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == *draw {
                    *cell = -99999999;
                } else if *cell > 0 {
                    score += *cell;
                }
            }
        }
        if check_won(&board) {
            return (turn, score * draw)
        }
    }
    (999999, -1)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let draws = read_draws(&stdin)?;
    println!("draws={:?}", draws);

    let mut best_board: (usize, i32) = (88888888, -1);
    let mut worst_board: (usize, i32) = (0, -1);
    while let Some(board) = read_board(&stdin)? {
        println!("\n{:?}", board);
        let result = check_board(&draws, board);
        println!("result={:?}", result);
        if result < best_board {
            println!("new best!");
            best_board = result
        }
        if result > worst_board {
            println!("new worst!");
            worst_board = result;
        }
    }
    println!("best_board={:?}", best_board);
    println!("worst_board={:?}", worst_board);
    Ok(())
}
