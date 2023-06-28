fn rook_protection_scores(position: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let mut scores = vec![vec![0; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            if position[row][col] != 'R' {
                continue;
            }

            let mut score = 0;

            // Check up
            for r in (0..row).rev() {
                if position[r][col] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check down
            for r in row+1..8 {
                if position[r][col] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check left
            for c in (0..col).rev() {
                if position[row][c] != ' ' {
                    score += 1;
                    break;
                }
                score += 1;
            }

            // Check right
            for c in col+1..8 {
                if position[row][c] != ' ' {
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