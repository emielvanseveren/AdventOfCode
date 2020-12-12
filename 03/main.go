package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	input,_ := ioutil.ReadFile("./03/input")

	// Part 1
	trees, pos := 0,0
	for _, l := range strings.Split(string(input), "\n"){
		if l[pos:pos+1] == "#" {
			trees++
		}
		pos = (pos + 3) % len(l)
	}
	fmt.Printf("Amount of trees: %d \n", trees)

	// Part 2
	p := [][]int{{1,1}, {3,1}, {5,1}, {7,1}, {1,2}}
	trees=1
	for i:=0;i<len(p);i++ {
		trees *= scanTree(p[i][0], p[i][1])
	}
	fmt.Printf("Amount of trees: %d \n", trees)
}


func scanTree(right int, down int) (trees int){
	input,_ := ioutil.ReadFile("./03/input")
	pos, downCount := 0, 0

	// Part 2
	for _, l := range strings.Split(string(input), "\n"){
		if down == 2 && downCount == 1 {
			downCount=0
			continue
		}
		downCount++

		if l[pos:pos+1] == "#" { trees++ }
		pos = (pos+right) % len(l)
	}
	return
}



