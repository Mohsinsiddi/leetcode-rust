impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        use std::collections::HashMap;
        let mut count_map: HashMap<String,i32> = HashMap::new();
        for a in arr.iter() {
            *count_map.entry(a.to_string()).or_insert(0) +=1;
        }
        let mut counter = k;
        for a in arr.iter() {
            let count = count_map.get(a).unwrap() ;
            if count == &1 { 
                counter-=1;
                if counter == 0 {
                    return a.to_string();
                }
            }
        } 
        "".to_string()
    }
}
