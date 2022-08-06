namespace Csharp.SlideWindow.Q3;

public class Solution1
{
    public int LengthOfLongestSubstring(string s)
    {
        int left = 0, right = 0, res = 0;
        var window = new Dictionary<char, int>();

        while (right < s.Length)
        {
            var c = s[right];
            right++;
            if (!window.TryAdd(c, 1))
            {
                window[c]++;
            }

            while (window[c] > 1)
            {
                var d = s[left];
                left++;
                window[d]--;
            }

            res = Math.Max(res, right - left);
        }

        return res;
    }
}