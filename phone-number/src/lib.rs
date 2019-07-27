#![warn(clippy::all)]
pub fn number(user_number: &str) -> Option<String> {
    let mut c: Vec<char> = user_number
        .chars()
        .filter(|x| match x {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
            _ => false,
        })
        .collect();
    if c[0] == '1' && c.len() == 11 {
        c.remove(0);
    }
    if c[0] == '0' || c[0] == '1' || c[3] == '0' || c[3] == '1' || c.len() != 10 {
        None
    } else {
        Some(c.iter().collect())
    }
}
