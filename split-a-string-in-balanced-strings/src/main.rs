impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut left_count = 0;
        let mut ans = 0;
        s.chars().fold(0,|all,x| {
            left_count += if x == 'R' { 1 } else { -1 };
            if left_count == 0 {
                return all +1;
            } 
            all
        })    
    }
}
