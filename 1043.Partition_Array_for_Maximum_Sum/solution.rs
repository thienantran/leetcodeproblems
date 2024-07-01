impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (arr.len(), k as usize);
        let mut dp = vec![0; n + 1];

        dp[1] = arr[0];

        for i in 1..n{
            let mut max_value = 0;
            let mut max_groups = 0;
            for j in 0..k.min(i + 1) {
                max_value = max_value.max(arr[i-j]);
                max_groups = max_groups.max((j + 1) as i32 * max_value + dp[i - j]);
            }
            dp[i + 1] = max_groups;
        }
        dp[n]
    }
}
