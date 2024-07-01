impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr
        .into_iter()
        .scan(0, |acc, el| {
            if el % 2 == 1 {
                *acc += 1;
                Some(*acc)
            } else {
                *acc =0;
                Some(*acc)
            }
        })
        .any (|x| x >= 3)
    }
}
