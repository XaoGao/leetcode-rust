fn roman_to_int(s: String) -> i32 {
    let roman_map = std::collections::HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut sum = 0;
    let arr = s.chars().collect::<Vec<char>>();
    for i in 0..arr.len() {
        let current = arr[i];
        if i + 1 < arr.len() {
            let next = arr[i + 1];
            if roman_map.get(&current).unwrap() < roman_map.get(&next).unwrap() {
                sum -= roman_map.get(&current).unwrap();
            } else {
                sum += roman_map.get(&current).unwrap();
            }
        } else {
            sum += roman_map.get(&current).unwrap();
        }
    }

    return sum;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("VI".to_string()), 6);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
