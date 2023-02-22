impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let first_half = nums.clone();
        let second_half = nums.clone();
        [first_half,second_half].concat()
    }
}