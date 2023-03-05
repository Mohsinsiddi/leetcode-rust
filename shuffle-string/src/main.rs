impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let chars:  Vec<char> = s.chars().collect();
        let mut ans = vec![String::from("");indices.len()]; 
        for i in 0..indices.len() {
              let index = indices[i] as usize;
              ans[index] = chars[i].to_string();
        }
        let mut res = String::from("");
        for s in ans {
            res += &s;
        }
        res
    }
}
