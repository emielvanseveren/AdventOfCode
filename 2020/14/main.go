package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
	"time"
)

func main() {
	input, err := ioutil.ReadFile("./14/input")
	check(err)

	go spinner(30*time.Millisecond) // fun concurrency :)

	var mask string
	mem1, part1 := make(map[int]int), 0
	mem2, part2 := make(map[int]int), 0
	for _, s := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		if s[:4] == "mask" {
			mask = s[7:]
			continue
		}

		// f-refs
		var address, value int
		fmt.Sscanf(s, "mem[%d] = %d", &address, &value)

		x := strings.Count(mask, "X")
		for i:= 0; i < 1<<x; i++ {     // 1 * 2 <- (x times)  left bitshift
			mask := strings.ReplaceAll(mask, "0", "x")
			for _, r := range fmt.Sprintf("%0*b", x, i) { // sprintf will write the result to a buffer instead of stdout which is than used in the for.
				mask = strings.Replace(mask, "X", string(r), 1)
			}
			addr := applyMask(strings.ReplaceAll(mask, "x", "X"), address)
			part2= part2+value-mem2[addr]
			mem2[addr] = value
		}
		value = applyMask(mask, value)
		part1 = part1+value-mem1[address]
		mem1[address] = value
	}


	fmt.Println("Sum part 1: ", part1)
	fmt.Println("Sum part2: ", part2)
}

func applyMask(mask string, value int) int {
	and, err := strconv.ParseUint(strings.ReplaceAll(mask, "X", "1"), 2, 0)
	check(err)
	or, err := strconv.ParseUint(strings.ReplaceAll(mask, "X", "0"), 2, 0)
	check(err)
	return value&int(and) | int(or)
}

// panic if errors
func check(err error){
	if err != nil {
		panic(err)
	}
}

/* for fun */
func spinner(delay time.Duration){
	for _, r:= range `-\|/` {
		fmt.Printf("\r%c", r)
		time.Sleep(delay)
	}
}