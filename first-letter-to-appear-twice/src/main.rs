impl Solution {
    pub fn repeated_character(s: String) -> char {
        use std::collections::HashSet;

        let mut hs = HashSet::new();
        let mut ans = char::default();
        for char in s.chars().collect::<Vec<char>>() {
            if hs.contains(&char) {
                ans = char;
                break;
            }
            hs.insert(char);
        }
        ans
    }
}
