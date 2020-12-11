package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	input, _ := ioutil.ReadFile("./07/input")

	bags := map[string]map[string]int{}
	for _, s := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		out := regexp.MustCompile(`(.+) bags contain`).FindStringSubmatch(s)[1]

		bags[out] = map[string]int{}
		for _, in := range regexp.MustCompile(`(\d+) (.+?) bag`).FindAllStringSubmatch(s, -1) {
			bags[out][in[2]], _ = strconv.Atoi(in[1])
		}
	}

	total := 0
	for b := range bags {
		if contains(bags, b, "shiny gold") {
			total++
		}
	}

	// part 1
	fmt.Println(total)
	// part 2
	fmt.Println(count(bags, "shiny gold"))
}

func contains(bags map[string]map[string]int, out, in string) bool {
	if _, ok := bags[out][in]; ok {
		return true
	}
	for out := range bags[out] {
		if contains(bags, out, in) { // recursive loop
			return true
		}
	}
	return false
}

func count(bags map[string]map[string]int, bag string) (total int) {
	for k, v := range bags[bag] {
		total += v * (count(bags, k) + 1)
	}
	return // returns total
}

