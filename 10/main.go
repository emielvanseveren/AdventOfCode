package main

import (
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

/* make this using channels */
func main() {
	input, _ := ioutil.ReadFile("./10/input")

	// strings to numbers
	adapters:= make([]int, 0)
	for _, s := range strings.Split(string(input), "\n") {
		adapter, _ 	:= strconv.Atoi(s)
		adapters = append(adapters, adapter)
	}
	// sort numbers high to low?
	sort.Ints(adapters)

	done := make(chan bool)
	diffChannel := make(chan int)
	resultChannel := make(chan int)

	go func(){
		defer close(diffChannel)
		defer close(resultChannel)

		diffs := make([]int, 4) // default capacity of 4

		select {
			case diff, ok := <-diffChannel:
				if !ok {
					return
				}
				diffs[diff]++
				case <- done: resultChannel <- diffs[1] * diffs[3]
		}
	}()

	go func(){
		i := 0

		for {
			if adapters[i+1] {
				diffChannel <- 1
			} else if adapters[i+2] {
				diffChannel <- 2
			} else if adapters[i+3] {
				diffChannel <- 3
				i += 3
			} else {
				diffChannel <- 3
				Close(done)
				return
			}
		}
	}()
	return
}


