impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        2 - s.chars().rev().eq(s.chars()) as i32
    }
}
