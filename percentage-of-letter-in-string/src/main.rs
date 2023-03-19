impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count_char = 0i32;
        for ch in s.chars() {
            if ch == letter {
                count_char +=1;
            }
        }
       ((count_char * 100)/s.len() as i32) as i32
    }
}
