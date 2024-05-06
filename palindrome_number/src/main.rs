fn is_palindrome(x: i32) -> bool {
    if x < 0 { 
        return false;
    }
    let number_as_str = x.to_string();
    let str_as_array: Vec<char> = number_as_str.chars().collect();

    for i in 0..number_as_str.len() / 2 {
        if str_as_array[i] != str_as_array[number_as_str.len() - i - 1] {
            return false;
        }
    }

    return true;
}
fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(1001));
        assert!(is_palindrome(0))
    }
}