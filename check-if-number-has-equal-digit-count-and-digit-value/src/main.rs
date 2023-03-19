impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = vec![0;10];
        let num_it = num.bytes().map(|b| (b - b'0') as usize);
        num_it.clone().for_each(|idx| count[idx] +=1);
        num_it.enumerate().all(|(i,data)| count[i] == data)
    }
}
