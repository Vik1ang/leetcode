package main

import (
	"fmt"
	"go_leetcode/hashtable/q1"
)

func main() {
	s := q1.Solution1{}
	res := s.TwoSum([]int{1, 2, 5, 7}, 9)
	fmt.Println(res)
}
