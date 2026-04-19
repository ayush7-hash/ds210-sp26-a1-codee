use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // Base case: game is over, return the score and a dummy move
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves();
        let mut best_move = moves[0];
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        for (x, y) in moves {
            // Apply move to a clone so we don't mutate the original
            let mut new_board = board.clone();
            new_board.apply_move((x, y), player);
            let next_player = match player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            let (score, _, _) = SolutionAgent::solve(&mut new_board, next_player, _time_limit);

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }

            // Alpha-beta style early exit: can't do better than winning
            if best_score == 1 && player == Player::X { break; }
            if best_score == -1 && player == Player::O { break; }
        }

        (best_score, best_move.0, best_move.1)
    }      
}

fn heuristic(board: &Board) -> i32 {
    // If the game is over, use the exact score
    if board.game_over() {
        return board.score();
    }

    let mut score = 0;

    // Check all rows
    for row in 0..5 {
        for col in 0..3 {
            score += evaluate_window(board, &[
                (row, col), (row, col+1), (row, col+2)
            ]);
        }
    }

    // Check all columns
    for col in 0..5 {
        for row in 0..3 {
            score += evaluate_window(board, &[
                (row, col), (row+1, col), (row+2, col)
            ]);
        }
    }

    // Check diagonals (top-left to bottom-right)
    for row in 0..3 {
        for col in 0..3 {
            score += evaluate_window(board, &[
                (row, col), (row+1, col+1), (row+2, col+2)
            ]);
        }
    }

    // Check diagonals (top-right to bottom-left)
    for row in 0..3 {
        for col in 2..5 {
            score += evaluate_window(board, &[
                (row, col), (row+1, col-1), (row+2, col-2)
            ]);
        }
    }

    score
}

fn evaluate_window(board: &Board, cells: &[(usize, usize)]) -> i32 {
    let mut x_count = 0;
    let mut o_count = 0;

    for &(row, col) in cells {
        match board.get_cells()[row][col] {
            Cell::X => x_count += 1,
            Cell::O => o_count += 1,
            Cell::Empty | Cell::Wall => {}
        }
    }

    // A window with both X and O can't be a three-in-a-row for either
    if x_count > 0 && o_count > 0 {
        return 0;
    }

    if x_count == 3 {
        return 10;      // completed three-in-a-row for X
    } else if x_count == 2 {
        return 3;       // two in a row, potential for X
    } else if x_count == 1 {
        return 1;
    }

    if o_count == 3 {
        return -10;     // completed three-in-a-row for O
    } else if o_count == 2 {
        return -3;
    } else if o_count == 1 {
        return -1;
    }

    0
}