#include <iostream>
#include <unordered_map>
#include <vector>

namespace q1 {
class Solution1 {
 public:
    std::vector<int> twoSum(std::vector<int>& nums, int target) {
        std::unordered_map<int, int> hash;
        for (int i = 0; i < nums.size(); ++i) {
            auto it = hash.find(target - nums[i]);
            if (it != hash.end()) {
                return {it->second, i};
            }
            hash[nums[i]] = i;
        }

        return {};
    }
};
}  // namespace q1

int main() {
    std::cout << "q1" << std::endl;
    return EXIT_SUCCESS;
}