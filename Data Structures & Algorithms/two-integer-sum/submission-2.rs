use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut imap = HashMap::new();
        for (ia, a) in nums.iter().enumerate() {
            let c = target - a;
            if let Some(&j) = imap.get(&c) {
                if ia != j {
                    return vec![j as i32, ia as i32];
                }
                
            }

            imap.insert(a, ia);
        }

        return vec![-1,-1];
    }
}
