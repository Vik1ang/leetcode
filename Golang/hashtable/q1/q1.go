package q1

type Solution1 struct{}

func (s *Solution1) TwoSum(nums []int, target int) []int {
	hashtable := map[int]int{}
	for index, val := range nums {
		if findIndex, ok := hashtable[target-val]; ok {
			return []int{findIndex, index}
		}
		hashtable[val] = index
	}
	return nil
}
