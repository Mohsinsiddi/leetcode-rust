impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let sumVa:i32 = nums.iter().sum();
        let mut digitSum:i32 = 0;
        for mut num in nums {
            while num!=0 {
                digitSum += num%10;
                num /=10;
            }
        }
     let ans = sumVa - digitSum;
      ans.abs()
    }
}
