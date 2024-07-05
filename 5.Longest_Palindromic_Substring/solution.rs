impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let sb = s.as_bytes();
        let mut pos = (0, 0);

        for i in 0..sb.len() * 2 {
            let (mut l, mut r) = (i / 2, i / 2 + (i % 2 != 0) as usize);
            while l <= r && l < sb.len() && r < sb.len() {
                if sb[l] == sb[r] {
                    if r - l > pos.1 - pos.0 { pos = (l, r) }
                } else {
                    break
                }
                l -= 1;
                r += 1;
            }
        }
        s[pos.0..=pos.1].to_string()
    }
}
