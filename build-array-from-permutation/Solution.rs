impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v:Vec<i32> = Vec::new();
        for num in nums.iter() {
         v.push(nums[*num as usize]);
        }
        v
    }
}