impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        for c in 'a'..='z' {
            if !sentence.contains(c) {
                return false;
            }
        }
        true
    }
}
