#include <string>
#include <unordered_map>
#include <unordered_set>

namespace q3 {
    class Solution1 {
    public:
        int lengthOfLongestSubstring(std::string s) {
            std::unordered_map<char, int> window;

            int left = 0, right = 0;
            int res = 0;

            while (right < s.size()) {
                char c = s[right];
                right++;
                // 更新窗口内数据
                window[c]++;
                // 判断左侧窗口是否要收缩
                while (window[c] > 1) {
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

    class Solution2 {
    public:
        int lengthOfLongestSubstring(std::string s) {
            int left = 0, right = 0, res = 0;
            std::unordered_set<char> window;
            for (int i = 0; i < s.length(); ++i) {
                char ch = s[i];
                while (window.count(ch) == 1) {
                    window.erase(s[left++]);
                }
                window.insert(ch);
                res = std::max(res, right - left + 1);
                right++;
            }

            return res;
        }
    };

    class Solution3 {
    public:
        int lengthOfLongestSubstring(std::string s) {
            int left = 0, res = 0;
            int window[128] = {0};


            for (int i = 0; i < s.length(); i++) {
                char ch = s[i];
                left = std::max(left, window[ch]);
                res = std::max(res, i - left + 1);
                window[ch] = i + 1;
            }

            return res;
        }
    };

}