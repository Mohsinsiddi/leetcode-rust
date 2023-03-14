impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for pattern in patterns.iter() {
            if word.contains(pattern) {
                ans +=1;
            }
        }
        ans
    }
}
