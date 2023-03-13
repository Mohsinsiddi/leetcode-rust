impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut counts =0;
        let mut bars = 0;

        for c in s.chars() {
            if c == '|' {
                bars +=1;
            }
            else if c == '*' && bars%2==0 {
                counts+=1;
            }
        }
        counts 
    }
}
