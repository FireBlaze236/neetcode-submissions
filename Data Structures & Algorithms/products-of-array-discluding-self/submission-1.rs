impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for (ia,a) in nums.iter().enumerate() {
            let mut prod = 1;
            for (ib,b) in nums.iter().enumerate() {
                if ia != ib {
                    prod *= b;
                }
            }
            res.push(prod);
        }

        res
    }
}
