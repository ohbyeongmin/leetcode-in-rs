pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one() {
        let nums = vec![2, 7, 11, 15];
        let target: i32 = 9;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn case_second() {
        let nums = vec![3, 2, 4];
        let target: i32 = 6;

        assert_eq!(two_sum(nums, target), vec![1, 2])
    }

    #[test]
    fn case_third() {
        let nums = vec![3, 3];
        let target: i32 = 6;

        assert_eq!(two_sum(nums, target), vec![0, 1])
    }
}
