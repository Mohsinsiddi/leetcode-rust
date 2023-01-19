impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 || strs[0].len() == 0 {
            return "".to_string();
        }
        let mut index = 0;
        for i in 0..strs[0].len() {
            let ch = strs[0].chars().nth(i);
            for str in &strs {
                if let Some(x) = str.chars().nth(i) {
                    if ch != Some(x) {
                         return strs[0][..i].to_string();
                    }
                } else {
                    return strs[0][..i].to_string();
                }
            }
            index = i;
        }
        strs[0][..=index].to_string()
    }
}
