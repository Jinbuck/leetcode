impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut count: i32 = 0;
        let mut num: i32 = n;
        while num != 0 {
            if num & 0x00000001 == 1 {
                count += 1;
            }
            num = num >> 1;
        }
        count
    }
}
