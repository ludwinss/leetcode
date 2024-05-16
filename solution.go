package main

import (
	"fmt"
)

func twoSum(nums []int, target int) []int {
	var mapNums = make(map[int]int)

	for idx, value := range nums {

		valor, ok := mapNums[target-value]
    if ok {
      return [] int {idx,valor}
    } 

    mapNums[value]=idx
    
	}
  return nil
}

func main() {
	fmt.Println(twoSum([]int{2, 7, 11, 15}, 9))
}
