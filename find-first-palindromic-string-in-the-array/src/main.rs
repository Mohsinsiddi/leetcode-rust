impl Solution {
    fn isPalindrom(s:&str) -> bool {
        let str_len = s.len();
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..str_len/2 {
            if chars[i] != chars[str_len-i-1] {
                return false;
            }
        }
        true
    }
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            let isPal = Self::isPalindrom(&word);
            if isPal {
                return String::from(word);
            }
        }
        String::from("")
    }
}
