impl Solution {

    pub fn check_grid(board : &Vec<Vec<char>>, x : usize, y : usize) -> bool {
        let mut bits : u16 = 0;
        for r in y..(y+3) {
            for c in x..(x+3) {
                let ch = board[r][c];
                if !ch.is_ascii_digit() {
                    continue;
                }
                let n = ch.to_digit(10).unwrap();

                if (bits & (1 << n)) >> n == 1 {
                    return false;
                } 
                bits |= (1 << n) as u16;
            }
        }

        return true;
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        for r in 0..rows {
            let mut bits : u16 = 0;
            for c in 0..cols {
                let ch = board[r][c];
                if !ch.is_ascii_digit() {
                    continue;
                }
                let n = ch.to_digit(10).unwrap();

                if (bits & (1 << n)) >> n == 1 {
                    return false;
                } 
                bits |= (1 << n) as u16;
            }
        }

        for c in 0..cols {
            let mut bits : u16 = 0;
            for r in 0..rows {
                let ch = board[r][c];
                if !ch.is_ascii_digit() {
                    continue;
                }
                let n = ch.to_digit(10).unwrap();

                if (bits & (1 << n)) >> n == 1 {
                    return false;
                } 
                bits |= (1 << n) as u16;
            }
        }


        return Self::check_grid(&board, 0, 0) && Self::check_grid(&board, 3, 0) && Self::check_grid(&board, 6, 0)
         && Self::check_grid(&board, 0, 3) && Self::check_grid(&board, 3, 3) && Self::check_grid(&board, 6, 3)
         && Self::check_grid(&board, 0, 6) && Self::check_grid(&board, 3, 6) && Self::check_grid(&board, 6, 6);
    }
}
