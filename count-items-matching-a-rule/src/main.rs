pub fn getCount(index:usize,items: Vec<Vec<String>>,rule_value: String) -> i32 {
        let mut ans = 0;
        for item in items {
           if rule_value == item[index] {
               ans +=1;
           }
        } 
        ans
}

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut ans =0;
          // for ruleKey == "type" : index = 0
          // for ruleKey == "color" : index = 1
          // for ruleKey == "name" : index = 2
          match rule_key.as_str() {
            "type" => {
                ans = getCount(0,items,rule_value);
            },
            "color" => {
                ans = getCount(1,items,rule_value);
            },
            "name" => {
                ans = getCount(2,items,rule_value);
            },
            _ => (),
        }
        ans
    }
}
