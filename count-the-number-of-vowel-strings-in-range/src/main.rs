impl Solution {
    fn isVowel(c:char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' 
    }
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        for i in left..=right {
            let chars = words[i as usize].chars().collect::<Vec<char>>();
            let chars_len = chars.len() - 1;
            if Self::isVowel(chars[0]) && Self::isVowel(chars[chars_len]) {
                 ans +=1;
            }
        }
        ans
    }
}
