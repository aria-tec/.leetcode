/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Create 1 new HashMap, Key: HashMap<[u8; 26], Value: Vec<String>.
       let mut res: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        // For each string in the input(use non & so that the strs can no longer be used):    
       for s in strs {
            
            // Create array: value 0, type u8 (8-bit each character can only be 0 - 255), length 26
            let mut count = [0u8; 26];

            // For each character in s  bytes:
            for c in s.bytes() {

                // substract a ASCII from c, then convert to usize, add 1 for each occurrence of the same character
                count[(c - b'a') as usize] += 1;
            }
            // check count entry, create if not already exists, enter s into count
            res.entry(count).or_default().push(s);
       }
       // Return the values of the HashMap as a Vec<Vec<String>>
       res.into_values().collect()
    }
}
// @lc code=end

