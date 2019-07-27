#![warn(clippy::all)]
pub fn number(user_number: &str) -> Option<String> {
    let (c, len) = user_number
        .chars()
        .fold((vec![], 0), |(mut acc, len), x| match x {
            '1' if len == 0 => (acc, len),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                acc.push(x);
                (acc, len + 1)
            }
            _ => (acc, len),
        });
    if c[0] == '0' || c[0] == '1' || c[3] == '0' || c[3] == '1' {
        None
    } else if len == 10 {
        let s: String = c.iter().collect();
        Some(s)
    } else {
        None
    }
}
