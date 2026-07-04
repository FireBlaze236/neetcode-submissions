impl Solution {
    pub fn get_count(s : &String) -> HashMap<char,i32>{
        let mut count = HashMap::new();
        for c in s.chars() {
            let cc = count.entry(c).or_insert(1);
            *cc += 1;
        }

        count
    }
    pub fn is_anagram(s1 : &String, s2 : &String) -> bool {
        Self::get_count(s1) == Self::get_count(s2)
    }
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut full_list = Vec::new();
        let mut found : HashMap<usize, bool> = HashMap::new();
        for (is,s) in strs.iter().enumerate() {
            if let Some(f) = found.get(&is) {
                continue;
            }
            let mut cur_list = Vec::new();

            cur_list.push(s.to_string());
            found.insert(is, true);
            for (i,t) in strs.iter().enumerate() {
                if let Some(f) = found.get(&i) {
                    continue;
                }
                if Self::is_anagram(s, t) {
                    cur_list.push(t.to_string());
                    found.insert(i, true);
                }
            }
            full_list.push(cur_list);
        }

        return full_list;
    }
}
