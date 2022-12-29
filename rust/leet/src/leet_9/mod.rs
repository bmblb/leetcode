fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false
    }
    else if x == 0 {
        return true
    }

    let mut tmp = x;
    // this one gives x2 speed boost
    let mut digits: Vec<i32> = Vec::with_capacity(10);
    
    loop {
        let digit = tmp % 10;
        
        digits.push(digit);
        
        tmp = tmp / 10;

        if tmp == 0 {
            break;
        }
    }

    let median = digits.len() / 2;

    for i in 0..median {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
    }

    true
}

// String solution is 6 times slower
// fn is_palindrome_str(x: i32) -> bool {
//     let string = x.to_string();
//     let string2 = string.chars().rev().collect::<String>();
//     if string == string2 {
//         return true;
//     }
//     false
// }

#[allow(dead_code)]
pub fn main() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(is_palindrome(12233221));
}
