use self::Player::*;

#[derive(Copy, Clone)]
enum Player {
    X,
    O,
    N,
}
/// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/
pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut grid = [Player::N; 9];

    moves.iter().enumerate().for_each(|(i, m)| {
        let row = m[0];
        let col = m[1];

        let player = if i % 2 == 0 { Player::X } else { Player::O };

        grid[(row * 3 + col) as usize] = player;
    });

    let result;

    match grid {
        [X, X, X, _, _, _, _, _, _]
        | [_, _, _, X, X, X, _, _, _]
        | [_, _, _, _, _, _, X, X, X]
        | [X, _, _, X, _, _, X, _, _]
        | [_, X, _, _, X, _, _, X, _]
        | [_, _, X, _, _, X, _, _, X]
        | [X, _, _, _, X, _, _, _, X]
        | [_, _, X, _, X, _, X, _, _] => {
            result = "A";
        }
        [O, O, O, _, _, _, _, _, _]
        | [_, _, _, O, O, O, _, _, _]
        | [_, _, _, _, _, _, O, O, O]
        | [O, _, _, O, _, _, O, _, _]
        | [_, O, _, _, O, _, _, O, _]
        | [_, _, O, _, _, O, _, _, O]
        | [O, _, _, _, O, _, _, _, O]
        | [_, _, O, _, O, _, O, _, _] => {
            result = "B";
        }
        _ => {
            if moves.len() == 9 {
                result = "Draw"
            } else {
                result = "Pending"
            }
        }
    }

    result.to_string()
}

#[test]
fn test_tictactoe() {
    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![2, 0],
            vec![1, 1],
            vec![2, 1],
            vec![2, 2]
        ]),
        "A"
    );

    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0]
        ]),
        "B"
    );

    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2]
        ]),
        "Draw"
    );

    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
        ]),
        "Pending"
    );
}
