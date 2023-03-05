impl Solution {
    pub fn interpret(command: String) -> String {
        let chars:Vec<char> = command.chars().collect();
        let mut ans = String::from("");
        for i in 0..chars.len() {
            if chars[i].to_string() == "G".to_string() {
                ans += "G";
            }
            else if chars[i].to_string() == "(".to_string() && 
chars[i+1].to_string() == ")".to_string() {
                ans += "o";
            }
            else if chars[i].to_string() == "(".to_string() && 
chars[i+1].to_string() == "a".to_string() {
                ans += "al";
            } 
            else {
                continue;
            }
        } 
        ans
    }
}
