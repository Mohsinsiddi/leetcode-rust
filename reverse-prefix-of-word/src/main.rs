impl Solution {
    fn rev_pre(s:&str,index:usize) -> String {
       let split_str = s.split_at(index+1);  
       let rev_chars_fh = split_str.0.chars().rev().collect::<Vec<char>>(); 
       let mut rev_chars_str =  rev_chars_fh.iter().collect::<String>();
       rev_chars_str += &split_str.1;
       rev_chars_str
    }
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut index = 0;
        for (i,e) in word.chars().enumerate() {
            if e == ch {
                index = i;
                break;
            }
        }
        Self::rev_pre(&word,index)   
    }
}
