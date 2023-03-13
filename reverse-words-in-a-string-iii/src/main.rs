impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ")
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
    }
}
