impl Solution {
    pub fn max_depth(s: String) -> i32 {
           let mut ans = 0;
           let mut count = 0;

           for char in s.chars() {
               match char {
                   '(' => {
                       count +=1;
                   },
                   ')' => {
                       count -=1;
                   },
                   _ => {
                       count+=0;
                   }
               }
               ans = std::cmp::max(count,ans);
           }
           ans
    }
}
