use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
    let vals = vec![1,1,1,1,1,1,1,1,1,1,2,2,3,3,3,3,4,4,4,4,4,5,8,8,10,10];

    let hm: HashMap<char, u64> = "AEIOULNRSTDGBCMPFHVWYKJXQZ"
                                    .char_indices()
                                    .map(|(i, ch)| (ch, vals[i]))
                                    .collect();

    word.to_ascii_uppercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| hm.get(&ch).unwrap())
        .sum()
}