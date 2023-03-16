impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0)+=1;
        }
        let mut hs: HashSet<usize> = HashSet::new();
        for v in map.values() {
            hs.insert(*v);
        }
        hs.len() == 1
    }
}
