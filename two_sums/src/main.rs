fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
