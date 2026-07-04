use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut count : HashMap<i32, bool> = HashMap::new();

        for i in nums {
            match count.get(&i) {
                Some(f) => {
                    return true;
                },
                None => {
                    count.insert(i, true);
                }
            }
        }

        false
    }
}
