package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
	"sync"
)

type Turns struct {
	numbersTurns map[int][]int
	turn int
	lastNumber int
}

func check(err error){
	if err != nil {
		panic(err)
	}
}

func (turns *Turns) setup(numbers []int) {
	turns.numbersTurns = make(map[int][]int)
	for i, n := range numbers {
		 turns.numbersTurns[n] = []int{i+1}
	}
	turns.lastNumber = numbers[len(numbers)-1]
	turns.turn = len(numbers)
}

func (turns *Turns) next() {
	turns.turn++
	t := turns.numbersTurns[turns.lastNumber]
	if len(t) < 2 {
		turns.lastNumber = 0
	} else {
		turns.lastNumber = t[len(t)-1] - t[len(t)-2]
	}
	turns.numbersTurns[turns.lastNumber] = append(turns.numbersTurns[turns.lastNumber], turns.turn)
	if len(turns.numbersTurns[turns.lastNumber]) > 2 {
		turns.numbersTurns[turns.lastNumber] = turns.numbersTurns[turns.lastNumber][1:]
	}

}

func main() {

	input, err := ioutil.ReadFile("./15/input")
	check(err)

	nums := make([]int,0)
	for _, s := range strings.Split(string(input), ","){
		num, err := strconv.Atoi(s)
		check(err)
		nums = append(nums, num)
	}

	// running both parts concurrent, overkill though :)
	var wg sync.WaitGroup
	wg.Add(2)
	go run(&wg, nums, 2020) 		// part 1
	go run(&wg, nums, 30000000) 	// part 2
	wg.Wait() // Wait for all the goroutines to finish before closing the main (go) routine.
}


func run(wg *sync.WaitGroup, nums []int, turnCount int){
	defer wg.Done()
	turns := Turns{}
	turns.setup(nums)
	for turns.turn < turnCount {
		// since the next depends on the previous numbers. We cannot run this concurrent.
		turns.next()
	}
	fmt.Println(turns.lastNumber)
}
