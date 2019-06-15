use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut num_field: Vec<Vec<u32>> = minefield.iter().map(|r| str_to_number(r)).collect();

    let row_len = num_field.len();
    if row_len == 0 {
        return Vec::new();
    }
    let col_len = num_field[0].len();

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

    let result = num_field.iter().map(|r| number_to_str(r)).collect();
    result
}

fn str_to_number(row: &str) -> Vec<u32> {
    row.chars()
        .map(|ch| match ch {
            '*' => 9,
            ' ' => 0,
            _ => 0, // TODO: Error handling
        })
        .collect()
}

fn number_to_str(row: &Vec<u32>) -> String {
    row.iter()
        .map(|n| match n {
            0 => ' ',
            9 => '*',
            _ => match char::from_digit(*n, 10) {
                None => ' ', // TODO: Better error handling
                Some(c) => c,
            },
        })
        .collect()
}
