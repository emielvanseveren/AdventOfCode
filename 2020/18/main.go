package main

import (
	"AdventOfCode/util"
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

func check(err error){
	if err != nil {
		panic(err)
	}
}

func main() {
	input, err := ioutil.ReadFile("./18/input")
	check(err)

	part1, part2:= 0,0
	reg := regexp.MustCompile(`\([^\(\)]+\)`)

	for _, l := range strings.Split(strings.TrimSpace(string(input)), "\n"){
		part1 += run(l, reg, 1, calc)
		part2 += run(l, reg, 1, plusFirst)
	}
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
func run(s string, re *regexp.Regexp, trim int, calc func(string) int) int {
	for re.MatchString(s){
		s = re.ReplaceAllStringFunc(s, func(s string) string {
			return strconv.Itoa(calc(s[trim: len(s) - trim]))
		})
	}
	return calc(s)
}

// calculate + before * expressions
func plusFirst(s string) int{
	return run(s, regexp.MustCompile(`\d+ \+ \d+`),0, calc)
}

func calc(expr string) (res int){
	fields := strings.Fields(expr)		// field[0]=num1; field[1]=operator; field[2]=num2
	res = util.GetInt(fields[0])		// Set first value already equal to res. If 0 and first operator is a multiplication, it would be wrong

	for i := 1; i<len(fields); i+=2 {
		num := util.GetInt(fields[i+1])
		switch fields[i] {
		case "+":
			res +=num
		case "*":
			res *=num
		}
	}
	return res
}

