use std::cmp::{min,Ordering};
impl Solution {
    fn calculate(curr:u8,prev:u8) -> i32 {
       let (small,big) = match curr.cmp(&prev) {
            Ordering::Greater => (prev, curr),
            Ordering::Less => (curr, prev),
            Ordering::Equal => return 0,
       };
       min(big - small, small + (26 - big)) as _
    }
    pub fn min_time_to_type(word: String) -> i32 {
        let mut count =0;
        let mut prev = 0_u8;
        for c in word.chars() {
           let curr = c as u8 - 97;
           count += Self::calculate(curr,prev) + 1;
           prev = curr;
        }
        count
    }
}
