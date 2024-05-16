fn valid_parentheses(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut stack: Vec<char> = Vec::new();
    for c in chars {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            if c == ')' && stack.pop() != Some('(') {
                return false;
            }
            if c == ']' && stack.pop() != Some('[') {
                return false;   
            }
            if c == '}' && stack.pop() != Some('{') {
                return false;
            }
        }
    }

    if stack.len() == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert!(valid_parentheses("()".to_string()));
        assert!(valid_parentheses("()[]{}".to_string()));
        assert!(!valid_parentheses("(){[".to_string()));
        assert!(valid_parentheses("{[]}".to_string()));
        assert!(!valid_parentheses("([)]".to_string()));
    }
}
