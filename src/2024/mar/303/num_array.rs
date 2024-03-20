struct NumArray {
    s: Vec<i32>
}

/**
 * 303. Range Sum Query - Immutable
 * Prefix Sum Solution:
 * Calculate the prefix sum of each position in the 
 * initiate method. The sum between left and right 
 * is prefix sum of position right+1 minus 
 * prefix sum of position left.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut s = vec![0; nums.len() + 1];
        for (i, &x) in nums.iter().enumerate() {
            s[i + 1] = s[i] + x;
        }
        Self { s }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.s[right as usize + 1] - self.s[left as usize]
    }
}