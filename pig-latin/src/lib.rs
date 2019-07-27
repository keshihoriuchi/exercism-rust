#![warn(clippy::all)]
pub fn translate(input: &str) -> String {
    input
        .split(' ')
        .map(process_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn process_word(input: &str) -> String {
    let cvec: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < cvec.len() {
        if i == 0 && cvec.len() >= 2 {
            if let ('x', 'r') | ('y', 't') = (cvec[0], cvec[1]) {
                break;
            }
        }
        if let 'a' | 'i' | 'e' | 'o' = cvec[i] {
            break;
        }
        if cvec[i] == 'u' && (i == 0 || cvec[i - 1] != 'q') {
            break;
        }
        i += 1;
    }
    let mut iy = 0;
    while iy < i && cvec[iy] != 'y' {
        iy += 1;
    }

    let mut s = String::new();
    cvec[i..].iter().for_each(|c| s.push(*c));
    cvec[iy..i].iter().for_each(|c| s.push(*c));
    cvec[..iy].iter().for_each(|c| s.push(*c));
    s.push_str("ay");
    s
}
