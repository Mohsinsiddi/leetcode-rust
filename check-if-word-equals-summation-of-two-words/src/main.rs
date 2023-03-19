impl Solution {
    fn calculate_sum_cal(word : &str) -> i32 {
        let mut res = "".to_string();
         for ch in word.chars() {
             res = res + &(ch as u8 - 'a' as u8).to_string();
         }
         res.parse::<i32>().unwrap()
    }
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
         Self::calculate_sum_cal(&first_word) + Self::calculate_sum_cal(&second_word) ==      Self::calculate_sum_cal(&target_word) 
    }
}
