impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut wealth = 0i32;

        for account in accounts {
            let sum = account.iter().sum();
            if sum > wealth {
                wealth = sum;
            }
        }
        wealth
    }
}
