use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // str to Vec<u32>
    let mut num_field: Vec<Vec<u32>> = minefield
        .iter()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '*' => 9,
                    ' ' => 0,
                    _ => panic!("Invalid char {}", ch),
                })
                .collect()
        })
        .collect();

    let row_len = num_field.len();
    if row_len == 0 {
        return Vec::new();
    }
    let col_len = num_field[0].len();

    // Count mines around each squares
    {
        let mut i = 0;
        while i < row_len {
            let mut j = 0;
            while j < col_len {
                if num_field[i][j] != 9 {
                    // upper left
                    if i != 0 && j != 0 && num_field[i - 1][j - 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // upper
                    if i != 0 && num_field[i - 1][j] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // upper right
                    if i != 0 && j + 1 != col_len && num_field[i - 1][j + 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // left
                    if j != 0 && num_field[i][j - 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // right
                    if j + 1 != col_len && num_field[i][j + 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // lower left
                    if i + 1 != row_len && j != 0 && num_field[i + 1][j - 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // lower
                    if i + 1 != row_len && num_field[i + 1][j] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                    // lower right
                    if i + 1 != row_len && j + 1 != col_len && num_field[i + 1][j + 1] == 9 {
                        num_field[i][j] = num_field[i][j] + 1;
                    }
                }
                j = j + 1;
            }
            i = i + 1;
        }
    }

    // Vec<u32> to String
    num_field
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| match n {
                    0 => ' ',
                    9 => '*',
                    _ => char::from_digit(*n, 10).unwrap(),
                })
                .collect()
        })
        .collect()
}
