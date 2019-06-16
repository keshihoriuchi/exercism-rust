#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    // Ex. images = [["1st line", "2nd line", "3rd line", "4th line"],
    //               ["5th line", "6th line", "7th line", "8th line"]]
    let images: Vec<Vec<&str>> = input
        .split("\n")
        .enumerate()
        .fold(Vec::new(), |mut v, (i, s)| {
            if i % 4 == 0 {
                v.push(Vec::new());
            }
            v[i / 4].push(s);
            v
        });

    // Ex. results = ["123", "456", "789"]
    let mut results = Vec::new();
    for rows in images {
        // validate row count
        if rows.len() % 4 != 0 {
            return Err(Error::InvalidRowCount(rows.len()));
        }

        // Ex. ocr_strs = [" _ | ||_|   ", "     |  |   "]
        let mut ocr_strs: Vec<String> = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            // validate column count
            let col_len = (*row).chars().count();
            if col_len % 3 != 0 {
                return Err(Error::InvalidColumnCount(col_len));
            }

            for (j, c) in (*row).chars().enumerate() {
                if i == 0 && j % 3 == 0 {
                    let mut s = String::with_capacity(12);
                    s.push(c);
                    ocr_strs.push(s)
                } else {
                    ocr_strs[j / 3].push(c)
                }
            }
        }

        // Ex. result = "123"
        let result: String = ocr_strs
            .iter()
            .map(|s| match s.as_ref() {
                " _ | ||_|   " => '0',
                "     |  |   " => '1',
                " _  _||_    " => '2',
                " _  _| _|   " => '3',
                "   |_|  |   " => '4',
                " _ |_  _|   " => '5',
                " _ |_ |_|   " => '6',
                " _   |  |   " => '7',
                " _ |_||_|   " => '8',
                " _ |_| _|   " => '9',
                _ => '?',
            })
            .collect();
        results.push(result)
    }

    // Ex. result = "123,456,789"
    let result = results
        .iter()
        .enumerate()
        .fold(String::new(), |mut s, (i, subs)| {
            if i != 0 {
                s.push(',')
            }
            s.push_str(subs);
            s
        });
    Ok(result)
}
