impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut ans = 0;
        for t in text.split(" ") {
            println!("{:?}",t);
            for c in broken_letters.chars() {
                if t.contains(c) {
                    ans-=1;
                     break;
                }
            }
            ans +=1;
        }
        ans
    }
}
