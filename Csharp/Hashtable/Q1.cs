namespace Hashtable.Q1;

public class Solution1
{
    public int[] TwoSum(int[] nums, int target)
    {
        var hashtable = new Dictionary<int, int>();
        for (int i = 0; i < nums.Length; i++)
        {
            if (hashtable.ContainsKey(target - nums[i]))
            {
                return new int[] { hashtable[target - nums[i]], i };
            }
            hashtable[nums[i]] = i;
        }
        return new int[] { -1, -1 };
    }
}