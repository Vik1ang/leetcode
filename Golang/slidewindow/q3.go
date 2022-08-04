package slidewindow

import (
	"math"
)

type Solution1 struct{}

func (s1 *Solution1) lengthOfLongestSubstring(s string) int {
	left := 0
	right := 0
	window := map[byte]int{}
	res := 0

	for right < len(s) {
		c := s[right]
		right++
		window[c]++

		for window[c] > 1 {
			d := s[left]
			left++
			window[d]--
		}

		res = int(math.Max(float64(res), float64(right-left)))
	}

	return res
}

type Solution2 struct{}

func (s2 *Solution2) lengthOfLongestSubstring(s string) int {
	left := 0
	right := 0
	res := 0
	window := map[byte]bool{}

	for i := 0; i < len(s); i++ {
		for window[s[i]] {
			delete(window, s[left])
			left++
		}
		window[s[i]] = true
		res = int(math.Max(float64(res), float64(right-left+1)))
		right++
	}

	return res
}

type Solution3 struct{}

func (s3 *Solution3) lengthOfLongestSubstring(s string) int {
	left := 0
	res := 0
	windows := make([]int, 128)

	for i := 0; i < len(s); i++ {
		ch := s[i]
		left = int(math.Max(float64(left), float64(windows[ch])))
		res = int(math.Max(float64(res), float64(i-left+1)))
		windows[ch] = i + 1
	}

	return res
}
