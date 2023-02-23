impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize; 
    let mut ans : Vec<i32> = Vec::new();   
    for i in 0..n {
       ans.push(nums[i]);
       ans.push(nums[n+ i ]);
    } 
    ans  
    }
}
