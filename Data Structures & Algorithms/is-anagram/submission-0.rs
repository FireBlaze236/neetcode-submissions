use std::collections::HashMap;

impl Solution {

    pub fn get_counts(s: String) -> HashMap<char, i32> {
        let mut count = HashMap::new();
        for c in s.chars() {
            let entry = count.entry(c).or_insert(1);
            *entry += 1;
        }

        return count;
    }
    pub fn is_anagram(s: String, t: String) -> bool {
        Self::get_counts(s) == Self::get_counts(t)
    }
}
