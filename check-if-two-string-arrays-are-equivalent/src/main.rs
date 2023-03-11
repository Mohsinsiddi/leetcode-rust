     pub fn getString(string: Vec<String>) -> String {
         let mut ans = String::from("");
         for s in string {
             ans += &s;
         }
         ans
     }
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let str1 = getString(word1);
        let str2 = getString(word2);
        str1.eq(&str2)
    }
}}
