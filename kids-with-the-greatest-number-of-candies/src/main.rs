impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> 
Vec<bool> {
        
        let maxValue = *candies.iter().max().unwrap();

        let mut ans = Vec::new();

        for candie in candies {
            if (candie + extra_candies)>= maxValue {
                ans.push(true);
            } else {
                ans.push(false);
            }
        }
        ans
    }
}
