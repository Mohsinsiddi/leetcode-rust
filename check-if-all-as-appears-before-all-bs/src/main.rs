impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut b_met = false;
        const A : char = 'a';
        const B : char = 'b';
        for ch in s.chars() {
            match ch {
                A => {
                   if b_met {
                       return false;
                   }
                },
                B => {
                    b_met = true;
                },
                _ => unreachable!() 
            }
        }
        true
    }
}
