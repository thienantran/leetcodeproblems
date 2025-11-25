class Solution:
    def maxArea(self, height: List[int]) -> int:
        left = 0
        right = len(height) - 1
        max_area = 0
        while left < right: 
            h_l = height[left]
            h_r = height[right]
            if h_l < h_r:
                current_area = h_l * (right-left)
                if current_area > max_area:
                    max_area = current_area
                while left < right and height[left] <= h_l:
                    left += 1
            else:
                current_area = h_r * (right-left)
                if current_area > max_area:
                    max_area = current_area
                while left < right and height[right] <= h_r:
                    right -= 1
        return max_area
