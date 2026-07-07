impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res : String = String::new();

        for (i, s) in strs.iter().enumerate() {
            res += "#";
            res += &(s.len()).to_string();
            res += "_";
            res += s;
        }

        println!("{}", res);

        return res;
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                let start_num = i + 1;
                let digit_str : String = s.chars().skip(start_num).take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(len) = digit_str.parse() {
                    if len == 0 {
                        res.push("".to_string());
                        continue;
                    }
                    let start_str = s.chars().skip(start_num).position(|c| !c.is_ascii_digit());
                    if let Some(x) = start_str {
                        let sidx = start_num + x + 1;
                        res.push(s.chars().skip(sidx).take(len).collect());
                    }
                }
            }
        }

        return res;
    }
}
