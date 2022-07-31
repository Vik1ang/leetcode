package hashtable.q1

class Solution1 {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        hashMapOf<Int, Int>().apply {
            nums.forEachIndexed { index, num ->
                val other = target - num
                if (containsKey(other)) {
                    val otherIndex = get(other) ?: 0
                    return intArrayOf(otherIndex, index)
                }
                put(num, index)
            }
        }
        return intArrayOf()
    }
}