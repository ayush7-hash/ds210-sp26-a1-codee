use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();
        let mut best_move = moves[0];
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        // Alpha-beta start values
        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        for (x, y) in moves {
            let mut new_board = board.clone();
            new_board.apply_move((x, y), player);

            let next_player = match player {
                Player::X => Player::O,
                Player::O => Player::X,
            };

            let score = minimax(&mut new_board, next_player, 0, 4, alpha, beta);
            //                  depth limit raised to 4 ──────────────↑

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                    // Update alpha at the top level too
                    if score > alpha {
                        alpha = score;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                    // Update beta at the top level too
                    if score < beta {
                        beta = score;
                    }
                }
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

fn minimax(
    board: &mut Board,
    player: Player,
    depth: u32,
    max_depth: u32,
    mut alpha: i32,   // ← ADDED
    mut beta: i32,    // ← ADDED
) -> i32 {

    // Base cases: game over OR depth limit reached
    if board.game_over() || depth == max_depth {
        return heuristic(board);
    }

    let moves = board.moves();

    match player {
        // ── X is MAXIMIZING ──────────────────────────────────────
        Player::X => {
            let mut best_score = i32::MIN;

            for (x, y) in moves {
                let mut new_board = board.clone();
                new_board.apply_move((x, y), player);

                let score = minimax(
                    &mut new_board,
                    Player::O,
                    depth + 1,
                    max_depth,
                    alpha,   // pass current alpha down
                    beta,    // pass current beta down
                );

                if score > best_score {
                    best_score = score;
                }

                // Update alpha: best X has found on this path
                if score > alpha {
                    alpha = score;
                }

                // PRUNE: O already has a better option elsewhere
                // O will never let us reach this branch
                if alpha >= beta {
                    break;  // ← CUT OFF remaining moves
                }
            }

            best_score
        }

        // ── O is MINIMIZING ──────────────────────────────────────
        Player::O => {
            let mut best_score = i32::MAX;

            for (x, y) in moves {
                let mut new_board = board.clone();
                new_board.apply_move((x, y), player);

                let score = minimax(
                    &mut new_board,
                    Player::X,
                    depth + 1,
                    max_depth,
                    alpha,   // pass current alpha down
                    beta,    // pass current beta down
                );

                if score < best_score {
                    best_score = score;
                }

                // Update beta: best O has found on this path
                if score < beta {
                    beta = score;
                }

                // PRUNE: X already has a better option elsewhere
                // X will never choose a path that lets O do this well
                if alpha >= beta {
                    break;  // ← CUT OFF remaining moves
                }
            }

            best_score
        }
    }
}

fn heuristic(board: &Board) -> i32 {
    if board.game_over() {
        return board.score() * 100;  // ← weight completed games heavily
    }

    let mut score = board.score() * 100;

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

    // Check diagonals (top-left to bottom-right ↘)
    for row in 0..3 {
        for col in 0..3 {
            score += evaluate_window(board, &[
                (row, col), (row+1, col+1), (row+2, col+2)
            ]);
        }
    }

    // Check diagonals (top-right to bottom-left ↙)
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
    let mut wall_count = 0;

    for &(row, col) in cells {
        match board.get_cells()[row][col] {
            Cell::X => x_count += 1,
            Cell::O => o_count += 1,
            Cell::Wall => wall_count += 1,
            Cell::Empty => {}
        }
    }

    // Wall in window = can never complete → worthless
    if wall_count > 0 {
        return 0;
    }

    // Both players present → neither can complete → worthless
    if x_count > 0 && o_count > 0 {
        return 0;
    }

    // Pure X window
    if o_count == 0 {
        return match x_count {
            3 => 10,
            2 => 3,
            1 => 1,
            _ => 0,
        };
    }

    // Pure O window
    if x_count == 0 {
        return match o_count {
            3 => -10,
            2 => -3,
            1 => -1,
            _ => 0,
        };
    }

    0
}
