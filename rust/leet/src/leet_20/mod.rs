fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}'|')'|']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }   
    return stack.is_empty();
}

pub fn main() {
    assert!(is_valid(String::from("()")));
    assert!(is_valid(String::from("()[]{}")));
    assert!(!is_valid(String::from("(]")));
}
