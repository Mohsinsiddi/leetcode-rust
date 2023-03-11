impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(' ' as u8, ' ' as u8);    
        let mut st = 'a' as u8;
        for b in key.as_bytes() {
            if map.get(&b).is_none() {
                map.insert(*b,st);
                st+=1;
            }
        }
        let mut res = String::with_capacity(message.len());
        for b in message.as_bytes() {
            res.push(*map.get(b).unwrap() as char);
        }
        res
    }
}
