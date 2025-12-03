class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:

        closest = float('inf')
        nums.sort()
        res = None
        for i in range(len(nums)):

            j = i + 1
            k = len(nums) - 1
            while j < k:

                total = nums[i] + nums[j] + nums[k]
                diff = abs(target - total)
                if diff == 0:
                    return total

                if diff < closest:
                    res = total
                    closest = diff

                if total > target:
                    k -= 1
                elif total < target:
                    j += 1

        return res
