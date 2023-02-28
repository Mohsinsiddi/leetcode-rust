impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        ans.push(first);
        for i in 0..encoded.len() {
            let currLen = ans.len();
            let nextItem = ans[currLen-1] ^ encoded[i];
            ans.push(nextItem);
        }
        ans
    }
}
