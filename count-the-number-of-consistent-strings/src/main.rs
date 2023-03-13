impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut hash = vec![false;26];

        for a in allowed.chars() {
            hash[((a as usize )-('a' as usize)) as usize] = true;
        }
        let mut result = 0;
        for word in words.iter() {
            let mut flag = true;
            for w in word.chars() {
                if !hash[((w as usize) -('a' as usize)) as usize] {
                    flag = false;
                    break;
                }
            }
            if flag {
                result +=1 
            }
        }
        result
    }
}
