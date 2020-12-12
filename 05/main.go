package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"sort"
	"strings"
)

func main() {
	// part 1
	seatIds := getSeatIds()
	fmt.Printf("Highest seat id: %d\n", getHighestSeatId(seatIds))

	// part 2
	sort.Ints(seatIds)
	for i, seatId := range seatIds {
		if i != len(seatIds)-1 && i!=0 {
			if  (seatIds[i+1] - seatIds[i-1]) != 2 {
				fmt.Printf("Checking seat %d \t prev: %d \t next: %d \n", seatId, seatIds[i-1], seatIds[i+1])
			}
		}
	}
   }

func getHighestSeatId(seatIds []int) int{
	highestSeatId := 0
	for _, seatId := range seatIds {
		if highestSeatId< seatId { highestSeatId = seatId }
	}
	return highestSeatId
}

func getSeatIds() []int {
	input, _ := ioutil.ReadFile("./05/input")
	highestSeatId, seatIds := 0, make([]int, 0)

	for _, s := range strings.Split(string(input), "\n") {
		seatId := getRowCount(s[0:7]) * 8 + getColCount(s[7:10])
		seatIds = append(seatIds, seatId)
		if seatId>highestSeatId { highestSeatId = seatId }
	}
	return seatIds
}


func getRowCount(rowInfo string) int {
	min, max := 0, 127

	for i:=0;i<len(rowInfo); i++ {
		char := []rune(rowInfo[i:i+1])

		if char[0] == 'B' {
			if  max - min == 1 {
				min = max
			} else {
				min += int(math.Round(float64(max - min) / 2))
			}
		}

		if char[0] == 'F' {
			if  max - min == 1 {
				max = min
			} else {
				max -= int(math.Round(float64(max - min) / 2))
			}
		}
	}
	return min
}

func getColCount(colInfo string) int {
	min, max := 0, 7

	for i:=0;i<len(colInfo); i++ {
		char := []rune(colInfo[i : i+1])

		if char[0] == 'L' {
			if max-min == 1 {
				max = min
			} else {
				max -= int(math.Round(float64(max-min) / 2))
			}
		}
		if char[0] == 'R' {
			if max-min == 1 {
				min = max
			} else {
				min += int(math.Round(float64(max-min) / 2))
			}
		}
	}
	return min
}