/**
 * 2580. Count Ways to Group Overlapping Ranges
 * Sort the ranges array by left value of each range.
 * Then count how many seperate ranges there are, asign
 * it to k. The count ways will be 2 powered by k.
 */
impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges = ranges;
        const MOD: i64 = 1_000_000_007;
        ranges.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = ranges.len();
        let mut i = 0;
        let mut res = 1;
        while i < n {
            let mut r = ranges[i][1];
            let mut j = i + 1;
            while j < n && ranges[j][0] <= r {
                r = r.max(ranges[j][1]);
                j += 1;
            }
            res = (res * 2) % MOD;
            i = j;
        }

        res as i32
    }
}