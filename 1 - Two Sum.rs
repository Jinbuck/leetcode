use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&find) = map.get(&complement) {
                return vec![index as i32, find as i32];
            }

            map.insert(num, index);
        }

        vec![]
    }
}
