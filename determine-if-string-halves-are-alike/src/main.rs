impl Solution {
    fn count_vowel(chars:Vec<char>) -> i32 {
        let vowels = "aeiouAEIOU";
        let mut count = 0;
         for c in chars {
             if vowels.contains(c) {
                 count+=1;
             }
         }
         count
    }

    fn is_alike(fh:Vec<char>,sh:Vec<char>) -> bool {
       Self::count_vowel(fh) == Self::count_vowel(sh)
    }

    pub fn halves_are_alike(s: String) -> bool {
        let fh_sh = s.split_at(s.len()/2);
        let fh_chars = fh_sh.0.chars().collect::<Vec<char>>();
        let sh_chars = fh_sh.1.chars().collect::<Vec<char>>();
        Self::is_alike(fh_chars,sh_chars)
    }
}
