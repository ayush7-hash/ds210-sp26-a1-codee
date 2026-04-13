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