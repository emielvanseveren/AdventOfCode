package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)


func main() {
	input,_ := ioutil.ReadFile("./13/input")
	split := strings.Split(string(input), "\n")
	startTime,_ := strconv.Atoi(split[0])

	minutes := startTime
	earliestBusId := -1
	busIds := make(map[int]int) // made this a map instead of a slice to be able to reuse this in part 2.
	for i, l := range strings.Split(split[1], ",") {
		if l == "x" {
			continue
		}
		busId,_:= strconv.Atoi(l)
		busIds[busId] = i
		if busId - (startTime % busId) < minutes {
			minutes = busId - (startTime % busId)
			earliestBusId = busId
		}
	}
	fmt.Println(minutes * earliestBusId)

	// Part 2
	min:= 0		// Minimum value
	mul := 1 	// Multiplier
	for i, id := range busIds {
		for (min+id)%i != 0 {
			min+= mul
		}
		mul *= i
	}
	fmt.Println(min)
}