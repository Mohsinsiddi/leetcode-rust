impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
       operations.iter()
       .fold(0, |acc, ops| {
            if ops.contains("+") {
                acc + 1
            }
            else {
                acc - 1
            }
       })
    }
}fn main() {
    println!("Hello, world!");
}
