impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut pos = vec![];
         let mut ans = vec![];
        for (i,e) in s.chars().enumerate() {
            if e == c {
                pos.push(i as i32);
            }
        }
        let mut min = 10000;
        for i in 0..s.len() {
            min = 1000;
            for index in &pos {
                let m = i32::abs(index-i as i32);
                if min > m {
                    min = m;
                }
            }
            ans.push(min);
        }
        ans
    }
}
