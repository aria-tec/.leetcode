/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
use std::collections::{HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();

        for &nm in &nums {
            // Check entry n, if not present, insert 0, evaluation result + 1
            // key:nm value;.or_insert(0) += 1;
            *counts.entry(nm).or_insert(0) += 1;
        }
        // create an empty nested vector (for freq[cn][nm])
        let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        for (nm, cn) in counts {
            // vector just want usize
            let i = cn as usize;
            // reversed, key = value, value = key
            freq[i].push(nm);
        }

        let mut result: Vec<i32> = Vec::new();

        // for i in length reverse freq (use len for find highest frequency)
        for i in (1..freq.len()).rev() {
            // for number in frequency
            for &nm in &freq[i] {
                // insert number to result
                result.push(nm);

                // if result length == k return result
                if result.len() == (k as usize) {
                    return result;
                }
            }
        }
        result
    }
}
// @lc code=end
