use std::{collections::HashMap};

fn roman_to_int(s: String) -> i32 {
    let numerals: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)
    ]);

    s.chars().map(|c| { *numerals.get(&c).unwrap() }).rev().scan(0, |state, x| {
        *state = if x < *state { -x } else { x };

        Some(*state)
    }).sum()
}

#[allow(dead_code)]
pub fn main() {
    assert_eq!(roman_to_int(String::from("III")), 3);
    assert_eq!(roman_to_int(String::from("LVIII")), 58);
    assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
}
