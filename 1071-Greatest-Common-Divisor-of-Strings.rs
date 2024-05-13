impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 == str2.clone() + &str1 {
            let mut gcm = if str1.len() > str2.len() {
                str2.len()
            } else {
                str1.len()
            };

            while gcm > 1 {
                if str1.len() % gcm == 0 && str2.len() % gcm == 0 {
                    break;
                }
                gcm -= 1;
            }
            return str1[0..gcm].to_string();
        } else {
            return "".to_string();
        }
    }
}
