// https://leetcode.com/problems/counting-words-with-a-given-prefix/
// 2185. Counting Words With a Given Prefix

fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        
    words
        .into_iter()
        .filter(|w| {pref.len() <= w.len() && w[..pref.len()] == pref})
        .collect::<Vec<String>>()
        .len() as i32
    
}

fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|word| word.starts_with(pref.as_str())).count() as _
}