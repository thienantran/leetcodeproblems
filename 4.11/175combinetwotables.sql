class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        # Initialize two pointers for the sliding window
        left, right = 0, 0
        # Initialize a set to store unique characters in the current substring
        char_set = set()
        # Initialize the maximum length of substring without repeating characters
        max_length = 0

        while right < len(s):
            # Check if the character at the right pointer is not in the set
            if s[right] not in char_set:
                # Add the character to the set
                char_set.add(s[right])
                # Update the maximum length if the current substring is longer
                max_length = max(max_length, right - left + 1)
                # Move the right pointer to the next position
                right += 1
            else:
                # If the character at the right pointer is already in the set,
                # remove the character at the left pointer from the set and
                # move the left pointer to the next position
                char_set.remove(s[left])
                left += 1

        return max_length
