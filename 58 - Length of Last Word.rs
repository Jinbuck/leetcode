impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut tokens: Vec<&str> = s.trim().split(" ").collect();
        let last_str = tokens.pop().unwrap();
        last_str.chars().count() as i32
    }
}
