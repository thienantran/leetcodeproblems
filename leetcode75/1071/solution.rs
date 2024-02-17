impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // Helper function to calculate GCD of two numbers
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        // Calculate the GCD of the lengths of the two strings
        let gcd_length = gcd(str1.len(), str2.len());

        // Extract the potential common divisor based on the GCD of string lengths
        let potential_divisor = &str1[0..gcd_length];

        // Check if potential_divisor is a common divisor of both str1 and str2
        if str1.as_bytes().chunks(gcd_length).all(|chunk| chunk == potential_divisor.as_bytes()) &&
           str2.as_bytes().chunks(gcd_length).all(|chunk| chunk == potential_divisor.as_bytes()) {
            potential_divisor.to_string()
        } else {
            "".to_string()
        }
    }
}
