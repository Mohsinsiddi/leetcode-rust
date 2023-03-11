impl Solution {
    pub fn sort_sentence(s: String) -> String {
       let mut ans = vec!["";s.split(" ").count()];
       for word in s.split(" ") {
           let (word,number ) = word.split_at(word.len()-1);
           ans[number.parse::<usize>().unwrap()-1] = word ;
       } 
       ans.join(" ")
    }
}
