impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut numbers = [nums1, nums2].concat();
        numbers.sort();
        let numbers_length = numbers.len();
        let median_index = numbers_length / 2;

        if numbers_length % 2 == 0 {
            let lower = median_index - 1;
            let upper = median_index;
            return (numbers[lower] as f64 + numbers[upper] as f64) / 2.0;
        } else {
            return numbers[median_index] as f64;
        }
        
    }
}
