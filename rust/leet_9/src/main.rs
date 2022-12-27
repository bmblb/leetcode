fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false
    }
    else if x == 0 {
        return true
    }

    let mut tmp = x;
    let mut digits: Vec<i32> = vec![];
    
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

fn main() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(is_palindrome(12233221));
}
