package q5

type Solution1 struct{}

func (s1 *Solution1) longestPalindrome(s string) string {
	n := len(s)
	dp := make([][]bool, n)
	for i, _ := range dp {
		dp[i] = make([]bool, n)
	}

	ans := ""

	// l 为循环遍历的字串长度
	for l := 0; l < n; l++ { // l为本次循环遍历的子串长度
		for i := 0; i+l < n; i++ {
			j := i + l
			if l == 0 {
				dp[i][j] = true
			} else if l == 1 {
				dp[i][j] = (s[i] == s[j])
			} else {
				dp[i][j] = (s[i] == s[j] && dp[i+1][j-1])
			}
			if dp[i][j] && l+1 > len(ans) {
				ans = s[i : j+1]
			}
		}
	}

	return ans
}
