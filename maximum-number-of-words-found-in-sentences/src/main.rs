impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut ans:i32 = 0;

        for sentence in sentences {
            let split = sentence.split(" ");
            let vec = split.collect::<Vec<&str>>();
            let wordsCount = vec.len() as i32;
            if wordsCount > ans  {
                ans = wordsCount ;
            }
        }
        ans
    }
}
