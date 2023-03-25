use std::collections::HashSet;
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes().windows(3)
        .filter(|subs| subs.iter().collect::<HashSet<&u8>>().len() == 3 )
        .count() as _
    }
}}
