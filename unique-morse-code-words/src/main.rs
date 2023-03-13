impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::{HashMap,HashSet};
        let table = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
         let mut char_map : HashMap<char,&str> = HashMap::new();
         for c in 'a'..='z' {
            let index = ((c as usize)- ('a' as usize) )as usize;
            char_map.insert(c,table[index]);
         }
         let mut ans = 0;
         let mut char_set : HashSet<String> = HashSet::new();
         for word in words.iter() {
             let mut transform_sentence = String::from("");
             for c in word.chars() {
                  transform_sentence += char_map.get(&c).unwrap();
             }
            char_set.insert(transform_sentence);
         }

         char_set.len() as i32

    }
}
