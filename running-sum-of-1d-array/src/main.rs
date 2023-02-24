impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        nums.iter().fold(0,|mut sum, &val| {
             sum += val;
             ans.push(sum);
             sum
        });
        ans
    }
}fn main() {
    println!("Hello, world!");
}
