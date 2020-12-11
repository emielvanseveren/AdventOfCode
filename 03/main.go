package main

import (
	"bufio"
	"log"
	"os"
	"fmt"
)

func main() {
	file, err := os.Open("./3/input")
	if err != nil {
		log.Fatalf("ERROR: %s", err)
	}
	defer file.Close()

	trees := 0
	pos := 0
	s := bufio.NewScanner(file)
	for s.Scan() {
		line := s.Text()
		if line[pos:pos+1] == "#" {
			trees++
		}
		pos = (pos + 3) % len(line)
	}
	fmt.Printf("Amount of trees: %d \n", trees) // 164

	p := [][]int{{1,1}, {3,1}, {5,1}, {7,1}, {1,2}}
	trees=1
	for i:=0;i<len(p);i++ {
		trees *= scanTree(p[i][0], p[i][1])
		fmt.Printf("%d \n", scanTree(p[i][0], p[i][1]))

	}
	fmt.Printf("Amount of trees: %d \n", trees) // 164
}


func scanTree(right int, down int) int{

	file, err := os.Open("./3/input")
	if err != nil {
		log.Fatalf("ERROR: %s", err)
	}
	defer file.Close()

	pos :=0
	trees :=0
	downCount :=0

	// Part 2
	s := bufio.NewScanner(file)
	for s.Scan(){
		if down == 2 && downCount == 1 {
			downCount=0
			continue
		}
		downCount++

		line := s.Text()

		if line[pos:pos+1] == "#" {
			trees++
		}
		pos = (pos+right) % len(line)
	}
	return trees
}



