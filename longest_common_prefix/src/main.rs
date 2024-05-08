fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs_as_chars: Vec<Vec<char>> = strs
        .clone()
        .into_iter()
        .map(|str| str.chars().collect())
        .collect();

    let shortest_str = strs
        .iter()
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let mut common_prefix = String::from("");
    for i in 0..shortest_str.len() {
       for str in strs_as_chars.iter() {
          if str[i] != shortest_str[i] {
            return common_prefix;
          } 
       }
       common_prefix.push(shortest_str[i]);
    }
    return common_prefix;
}

fn main() {
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }

    #[test]
    fn test_longest_common_prefix_empty() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "");
    }
}
