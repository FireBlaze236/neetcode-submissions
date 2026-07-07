impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = Vec::new();
        let mut prod = 1;
        for (ia,a) in nums.iter().enumerate() {
            if ia == 0 {
                continue;
            }
            prod *= nums[ia - 1];     
            prefix.push(prod);
        }

        prod = 1;
        let mut suffix = Vec::new();
        for (ia, a) in nums.iter().rev().enumerate() {
            if ia == 0 {
                continue;
            }
            prod *= nums[nums.len() - ia];     
            suffix.push(prod);
        }
        suffix.reverse();

        //println!("{:?}, {:?}", prefix, suffix);

        let mut res = Vec::new();

        for (ia,a) in nums.iter().enumerate() {
            if ia == 0 {
                res.push(suffix[0]);
            }
            else if ia == nums.len() - 1 {
                res.push(prefix[nums.len() - 1 - 1])
            } else {
                res.push(suffix[ia] * prefix[ia - 1])
            }
        }

        res
    }
}
