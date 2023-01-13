use std::{cmp::max, collections::{HashSet,HashMap}};

#[allow(dead_code)]
fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0 as usize;

    for i in 0..s.len() {
        let mut map: HashSet<char> = HashSet::with_capacity(50);

        s.chars().skip(i).all(|symbol| {
            if map.contains(&symbol) {
                false
            }
            else {
                map.insert(symbol);
                true
            }
        });

        longest = max(map.len(), longest);
    }

    longest as i32
}

// This is a proper solution taken from leet code. Plenty of times faster
// and still robust
fn length_of_longest_substring_best(s: String) -> i32 {
    let mut m = HashMap::new();
    let mut ans = 0;
    let mut before = -1;
    let mut current = 0;
    for c in s.chars() {
        if let Some(last) = m.insert(c, current) {
            before = max(before, last);
        }
        ans = max(ans, current - before);
        current += 1;
    }
    ans
}

pub fn main() {
    assert_eq!(length_of_longest_substring_best(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring_best(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring_best(String::from("pwwkew")), 3);
    assert_eq!(length_of_longest_substring_best(String::from("a")), 1);
}
