impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let chars : Vec<char> = s.chars().collect();
        let mut ans = vec![];

        let start_col:char = chars[0];
        let end_col : char = chars[3];
        let start_row: char = chars[1];
        let end_row : char = chars[4];

        for col in start_col..=end_col {
            for row in start_row..=end_row {
                let mut s = col.to_string();
                s+=&row.to_string();
                ans.push(s);
            }
        }
        ans
    }
}
