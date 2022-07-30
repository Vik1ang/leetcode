from typing import List


class Solution1:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashtable = dict()
        for index, num in enumerate(nums):
            if target - num in hashtable:
                return [hashtable[target - num], index]
            hashtable[nums[index]] = index
        return []
