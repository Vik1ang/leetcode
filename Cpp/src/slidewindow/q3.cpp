#include <string>
#include <unordered_map>

namespace q3 {
    class Solution1 {
    public:
        int lengthOfLongestSubstring(std::string s) {
            std::unordered_map<char, int> window;

            int left = 0, right = 0;
            int res = 0;

            while (right < s.size())
            {
                char c = s[right];
                right++;
                // 更新窗口内数据
                window[c]++;
                // 判断左侧窗口是否要收缩
                while (window[c] > 1)
                {
                    char d = s[left];
                    left++;
                    // 进行窗口内数据的更新
                    window[d]--;
                }
                // 更新答案
                res = std::max(res, right - left);
            }

            return res;
        }
    };
}