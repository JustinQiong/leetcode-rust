/**
 * 1793. Maximum Score of a Good Subarray
 * Two pointers solution:
 * Point l point to the left of the subarray.
 * Point r point to the right of the subarray.
 * Both of l an d r point to k initially.
 * Greedily move l to left if l-- is greater than r++.
 * Otherwise move r to right if r++ is greater than l--.
 * Calculate the score from l to r and find the maxmium score.
 */
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let (mut l, mut r) = (k, k);
        let mut ans = 0;
        let mut low = nums[k as usize];
        while l >= 0 && r < n {
            low = low.min(nums[l as usize]).min(nums[r as usize]);
            ans = ans.max((r-l+1)*low);
            if l > 0 && r < n-1 {
                if nums[l as usize - 1] > nums[r as usize + 1] {
                    l -= 1;
                } else {
                    r += 1;
                }
            } else if l > 0 {
                l -= 1;
            } else {
                r += 1
            }
        }

        ans
    }
}