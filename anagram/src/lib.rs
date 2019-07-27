#![warn(clippy::all)]
use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // 文字の出現回数のMapを作って比較する
    let word_low = &word.to_lowercase();
    let word_map = analyze(word_low);
    let mut result: HashSet<&'a str> = HashSet::new();
    possible_anagrams.iter().for_each(|s| {
        let s_low = &s.to_lowercase();
        if s_low != word_low && word_map == analyze(s_low) {
            result.insert(s);
        }
    });
    result
}

fn analyze(w: &str) -> HashMap<char, u32> {
    let mut word_map: HashMap<char, u32> = HashMap::new();
    w.chars().for_each(|c| {
        let v = match &word_map.get(&c) {
            None => 0,
            Some(v) => *v + 1,
        };
        word_map.insert(c, v);
    });
    word_map
}
