impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            let mut count = 0;
            for j in 0..nums.len() {
                if nums[j] < nums[i] && j!=i {
                    count+=1;
                }       
            }
            ans.push(count);
        }
        ans
    }
}
