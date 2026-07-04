impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (ia, a) in nums.iter().enumerate() {
            for (ib, b) in nums.iter().enumerate() {
                if a + b == target && ia != ib {
                    return vec![ia as i32, ib as i32];
                }
            }
        }

        return vec![-1,-1];
    }
}
