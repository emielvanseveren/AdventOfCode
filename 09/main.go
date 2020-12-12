package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("./09/input")
	preamble := 25
	nums := make([]int, 0)

	for _, s := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		num, _ := strconv.Atoi(s)
		nums = append(nums, num)
	}

	for i, num := range nums {
		if i > preamble {
			if !summable(num, nums[i-preamble:i]) {
				fmt.Println(num)
				part2(num, nums[:i-1])
				return
			}
		}
	}

}

func part2(num int, nums []int) (min, max int){
	start, end := 0, 1

	sum := nums[start] + nums[end]
	for sum != num {
		if sum < num {
			end++
			sum += nums[end]
		}
		if sum > num {
			sum -= nums[start]
			start++
		}
	}

	min, max = minMax(nums[start: end+1])
	fmt.Print(min+max)
	return
}

func summable(num int, nums []int) bool{
	for i:=0; i<len(nums); i++ {
		for j:=1; j<len(nums); j++ {
			if num==8 {
				fmt.Println("dit")
			}

			if nums[i] + nums[j] == num {
				return true
			}
		}
	}
	return false
}

func minMax(nums []int ) (min, max int) {
	min, max = nums[0], nums[0]

	for _, num := range nums {
		if num < min {
			min = num
		}
		if num > max {
			max = num
		}
	}
	return
}
