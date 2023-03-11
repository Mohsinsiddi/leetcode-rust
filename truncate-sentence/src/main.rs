impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut ans : String = String::from("");
        for (index, item) in s.split(" ").enumerate() {
            if index as i32 == k {
                break;
            }
            let space = " ";
            ans = ans + &item + &space;
        }
       ans.trim().to_string()
    }
}
