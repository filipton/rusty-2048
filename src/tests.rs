#[cfg(test)]
mod test {
    use crate::BoardData;

    /*
    #[test]
    fn test_move_horizontal() {
        let mut board = BoardData { board: [[0; 4]; 4] };
        board.board[0][0] = 2;
        board.board[1][0] = 2;
        board.board[2][0] = 2;
        board.board[3][0] = 2;

        board.print_board();
        board.move_horizontal(true);
        board.print_board();

        assert_eq!(board.board[0][0], 0);
        assert_eq!(board.board[1][0], 0);
        assert_eq!(board.board[2][0], 4);
        assert_eq!(board.board[3][0], 4);

        board.move_horizontal(false);
        board.print_board();

        assert_eq!(board.board[0][0], 8);
        assert_eq!(board.board[1][0], 0);
        assert_eq!(board.board[2][0], 0);
        assert_eq!(board.board[3][0], 0);
    }

    #[test]
    fn test_move_horizontal_2() {
        let mut board = BoardData { board: [[0; 4]; 4] };
        board.board[0][0] = 2;
        board.board[1][0] = 0;
        board.board[2][0] = 2;
        board.board[3][0] = 0;

        board.print_board();
        board.move_horizontal(false);
        board.print_board();

        assert_eq!(board.board[0][0], 4);
        assert_eq!(board.board[1][0], 0);
        assert_eq!(board.board[2][0], 0);
        assert_eq!(board.board[3][0], 0);

        board.move_horizontal(true);
        board.print_board();

        assert_eq!(board.board[0][0], 0);
        assert_eq!(board.board[1][0], 0);
        assert_eq!(board.board[2][0], 0);
        assert_eq!(board.board[3][0], 4);
    }

    #[test]
    fn test_move_vertical() {
        let mut board = BoardData { board: [[0; 4]; 4] };
        board.board[0][0] = 2;
        board.board[0][1] = 2;
        board.board[0][2] = 2;
        board.board[0][3] = 2;

        board.print_board();
        board.move_vertical(true);
        board.print_board();

        assert_eq!(board.board[0][0], 0);
        assert_eq!(board.board[0][1], 0);
        assert_eq!(board.board[0][2], 4);
        assert_eq!(board.board[0][3], 4);

        board.move_vertical(false);
        board.print_board();

        assert_eq!(board.board[0][0], 8);
        assert_eq!(board.board[0][1], 0);
        assert_eq!(board.board[0][2], 0);
        assert_eq!(board.board[0][3], 0);
    }

    #[test]
    fn test_move_vertical_2() {
        let mut board = BoardData { board: [[0; 4]; 4] };
        board.board[0][0] = 2;
        board.board[0][1] = 0;
        board.board[0][2] = 2;
        board.board[0][3] = 0;

        board.print_board();
        board.move_vertical(false);
        board.print_board();

        assert_eq!(board.board[0][0], 4);
        assert_eq!(board.board[0][1], 0);
        assert_eq!(board.board[0][2], 0);
        assert_eq!(board.board[0][3], 0);

        board.move_vertical(true);
        board.print_board();

        assert_eq!(board.board[0][0], 0);
        assert_eq!(board.board[0][1], 0);
        assert_eq!(board.board[0][2], 0);
        assert_eq!(board.board[0][3], 4);
    }

    #[test]
    fn test_move_horizontal_vertical() {
        let mut board = BoardData { board: [[0; 4]; 4] };
        board.board[0][0] = 2;
        board.board[1][0] = 2;
        board.board[2][0] = 2;
        board.board[3][0] = 2;

        board.board[0][1] = 2;
        board.board[1][1] = 2;
        board.board[2][1] = 2;
        board.board[3][1] = 2;

        board.board[0][2] = 2;
        board.board[1][2] = 2;
        board.board[2][2] = 2;
        board.board[3][2] = 2;

        board.board[0][3] = 2;
        board.board[1][3] = 2;
        board.board[2][3] = 2;
        board.board[3][3] = 2;

        board.print_board();
        board.move_horizontal(true);
        board.print_board();

        board.move_vertical(true);
        board.print_board();

        board.move_horizontal(false);
        board.print_board();

        board.move_vertical(false);
        board.print_board();

        assert_eq!(board.board[0][0], 32);

        let mut sum = 0;
        for i in 0..4 {
            for j in 0..4 {
                sum += board.board[i][j];
            }
        }

        assert_eq!(sum, 32);
    }
    */
}
