impl Solution {
    fn checkIfOnlyDigit(s:&str) -> bool {
        for c in s.chars() {
            if c as u8 > 57 {
                return false;
            }
        }
        true
    }
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut ans = 0;
        for s in strs {
            let isDigitOnly = Self::checkIfOnlyDigit(&s);
            println!("{:?} is {:?}",s , isDigitOnly);
            if !isDigitOnly {
                ans = std::cmp::max(ans,s.len());
            } else {
                ans = std::cmp::max(ans,s.parse::<usize>().unwrap());
                println!("{:?}",s.parse::<usize>().unwrap());
            }
        }
        ans as i32
    }
}
