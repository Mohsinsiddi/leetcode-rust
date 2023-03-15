impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = std::collections::HashMap::new();
        for i in 0..paths.len() {
            map.insert(paths[i][0].as_str(), paths[i][1].as_str());
        }
        let mut cur = paths[0][0].as_str();
        loop {
            match map.get(cur) {
                Some(city) => cur = city,
                _ => break cur.to_string(),
            }
        }
    }
}
