impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        
        for (i,e) in names.into_iter().enumerate() {
            map.insert(heights[i],e);
        }
        map.into_values().rev().collect()
    }
}
