impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut ans = 0i32;
        for stone in stones.chars() {
            if jewels.contains(stone) {
                ans +=1;
            }
        }
        ans
    }
}
