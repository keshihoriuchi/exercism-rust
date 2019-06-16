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

    if images[0].len() % 4 != 0 {
        return Err(Error::InvalidRowCount(images[0].len()));
    }

    let col_len = images[0][0].chars().collect::<Vec<char>>().len();
    if col_len % 3 != 0 {
        return Err(Error::InvalidColumnCount(col_len));
    }

    // Ex. results = ["123", "456", "789"]
    let mut results = Vec::new();
    for rows in images {
        // Ex. ocr_strs = [" _ | ||_|   ", "     |  |   "]
        let mut ocr_strs: Vec<String> = Vec::new();
        for (i, row) in rows.iter().enumerate() {
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
        let mut result = String::new();
        for s in ocr_strs {
            let c = match s.as_ref() {
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
            };
            result.push(c)
        }
        results.push(result)
    }

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
