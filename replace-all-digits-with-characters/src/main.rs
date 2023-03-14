impl Solution {
    fn shift(c:char,digit:i32) -> char {
        ((c as u8) + digit as u8) as char
    }
    pub fn replace_digits(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        for i in (0..chars.len()-1).step_by(2) {
            let old_c = chars[i];
            let shift_by = (chars[i+1].to_string()).parse::<i32>().unwrap();
            let l_c = Self::shift(old_c,shift_by);
            chars[i+1] = l_c;
        } 
        chars.into_iter().collect()
    }
    
}
