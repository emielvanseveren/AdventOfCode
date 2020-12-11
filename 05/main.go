package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
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
		if(highestSeatId< seatId) {
			highestSeatId = seatId
		}
	}
	return highestSeatId
}

func getSeatIds() []int {
	file, err := os.Open("./5/input")
	var seatIds []int

	if err != nil {
		log.Fatal("ERROR: %s", err)
	}

	s := bufio.NewScanner(file)
	highestSeatId := 0

	for s.Scan() {
		seatId := getRowCount(s.Text()[0:7]) * 8 + getColCount(s.Text()[7:10])
		seatIds = append(seatIds, seatId)
		if seatId>highestSeatId {
			highestSeatId = seatId
		}
	}
	return seatIds
}


func getRowCount(rowInfo string) int {
	// max 127
	max := 127
	min := 0

	for i:=0;i<len(rowInfo); i++ {
		char := rowInfo[i:i+1]

		if char == "B" {
			if  max - min == 1 {
				min = max
			} else {
				min += int(math.Round(float64(max - min) / 2))
			}
		}

		if char == "F" {
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
	min := 0
	max := 7

	for i:=0;i<len(colInfo); i++ {
		char := colInfo[i : i+1]

		if char == "L" {
			if max-min == 1 {
				max = min
			} else {
				max -= int(math.Round(float64(max-min) / 2))
			}
		}
		if char == "R" {
			if max-min == 1 {
				min = max
			} else {
				min += int(math.Round(float64(max-min) / 2))
			}
		}
	}
	return min
}