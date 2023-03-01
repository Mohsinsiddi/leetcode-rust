impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        for mut i in (0..nums.len()-1).step_by(2) {
            if nums.len()==0 {
                return ans;
            }
            for j in 0..nums[i] {
              ans.push(nums[i+1]);
            }
        }
        ans
    }
} 
