use crate::position::Position;

pub fn bishop_scores() -> Vec<Vec<i32>> {
    let mut scores = vec![vec![0; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            let mut score = 0;

            // Check diagonal up and to the right
            for i in 1..8 {
                let r = row as i32 + i;
                let c = col as i32 + i;
                if r >= 8 || c >= 8 {
                    break;
                }
                score += 1;
            }

            // Check diagonal up and to the left
            for i in 1..8 {
                let r = row as i32 + i;
                let c = col as i32 - i;
                if r >= 8 || c < 0 {
                    break;
                }
                score += 1;
            }

            // Check diagonal down and to the right
            for i in 1..8 {
                let r = row as i32 - i;
                let c = col as i32 + i;
                if r < 0 || c >= 8 {
                    break;
                }
                score += 1;
            }

            // Check diagonal down and to the left
            for i in 1..8 {
                let r = row as i32 - i;
                let c = col as i32 - i;
                if r < 0 || c < 0 {
                    break;
                }
                score += 1;
            }

            scores[row][col] = score;
        }
    }

    scores
}

fn bishop_protection_scores(position: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut scores = vec![vec![0; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            if position[row][col] != 'B' {
                continue;
            }

            let mut score = 0;

            // Check diagonal up and to the right
            for i in 1..8 {
                let r = row as i32 + i;
                let c = col as i32 + i;
                if r >= 8 || c >= 8 {
                    break;
                }
                if position[r as usize][c as usize] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check diagonal up and to the left
            for i in 1..8 {
                let r = row as i32 + i;
                let c = col as i32 - i;
                if r >= 8 || c < 0 {
                    break;
                }
                if position[r as usize][c as usize] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check diagonal down and to the right
            for i in 1..8 {
                let r = row as i32 - i;
                let c = col as i32 + i;
                if r < 0 || c >= 8 {
                    break;
                }
                if position[r as usize][c as usize] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check diagonal down and to the left
            for i in 1..8 {
                let r = row as i32 - i;
                let c = col as i32 - i;
                if r < 0 || c < 0 {
                    break;
                }
                if position[r as usize][c as usize] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            scores[row][col] = score;
        }
    }

    scores
}