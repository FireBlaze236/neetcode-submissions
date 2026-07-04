impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();

        for i in nums {
            let entry = count.entry(i).or_insert(1);
            *entry += 1;
        }

        let mut entries : Vec<_> = count.iter().collect();

        entries.sort_by_key(|&(key, value)| value);


        let mut res = Vec::new();
        for i in 0..k {
            if let Some(v) = entries.pop() {
                res.push(*v.0);
            }
        }


        res
    }
}
