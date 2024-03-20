/**
 * 1969. Minimum Non-Zero Product of the Array Elements
 * Greedy solution:
 * Array is like 1, 2, 3...2**p-3, 2**p-2, 2**p-1.
 * Greedily make the smallest number smaller and
 * the largest number larger will get minimum product.
 * So we make 2 to 1 and 2**p-3 to 2**p-2.
 * Make 3 to 1 and 2**p-4 to 2**p-2.
 * So at last the array will be 1, 1, 1 ...2**p-2, 2**p-2, 2**p-2, 2**p-1.
 */
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        fn fast_pow(mut x: i64, mut n: i64, mod_val: i64) -> i64 {
            let mut res = 1;
            while n != 0 {
                if n & 1 == 1 {
                    res = (res * x) % mod_val;
                }
                x = (x * x) % mod_val;
                n >>= 1;
            }
            res
        }

        if p == 1 {
            return 1;
        }

        const MOD: i64 = 1_000_000_007;
        let x = fast_pow(2, p as i64, MOD) - 1;
        let y = 1i64 << (p - 1);
        (fast_pow(x - 1, y - 1, MOD) * x % MOD) as i32
    }
}