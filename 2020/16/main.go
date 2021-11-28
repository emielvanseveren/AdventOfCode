package main

import (
	"AdventOfCode/util"
	"fmt"
	"github.com/gitchander/permutation"
	"io/ioutil"
	"strings"
)

func check(err error){
	if err != nil {
		panic(err)
	}
}

type Rule struct {
	description string
	ranges [2][2]int
}

func (r *Rule) numIsValid(num int) bool {
	// Here for debugging
	if (num >= r.ranges[0][0] && num <= r.ranges[0][1]) || (num >= r.ranges[1][0] && num <= r.ranges[1][1]) {
		return true
	}
	return false
}


func main() {

	conditions := parseConditions()
	part2(conditions, part1(conditions))

	// myTicket := []int{61,101,131,127,103,191,67,181,79,71,113,97,173,59,73,137,139,53,193,179}

	fmt.Println("res", 0)
}

func part1(rules []Rule) (validTickets [][]int){
	errorRate := 0

	input, err := ioutil.ReadFile("./16/nearby_tickets")
	check(err)
	var valid bool

	for _, s := range strings.Split(string(input), "\n"){
		valid = true
		ticketNumbers := make([]int, 0)
		for _, n := range strings.Split(s, ","){
			num := util.GetInt(n)

			for _, rule := range rules {
				if rule.numIsValid(num) {
					valid = false
					errorRate += num
					break
				}
			}
			ticketNumbers = append(ticketNumbers, num)
		}

		if valid {
			validTickets = append(validTickets, ticketNumbers)
		}

	}
	fmt.Println("Error rate: ", errorRate)
	return
}

func part2(rules []Rule, tickets[][]int){
	// add my ticket to the validTickets
	tickets = append(tickets, []int{61,101,131,127,103,191,67,181,79,71,113,97,173,59,73,137,139,53,193,179})

	// make an array of all possible condition orders
	resChan := make(chan[]Rule)
	p := permutation.New(permutation.MustAnySlice(rules))

	for p.Next() {
		go isCorrect(resChan, rules, tickets)
	}
	defer close(resChan)

	go multiplyDepartureFields(<-resChan, tickets[len(tickets)-1])
}

func isCorrect(res chan<-[]Rule, rules []Rule, tickets[][]int) {
	for _, ticket := range tickets {
		for i, num := range ticket {
			if !(num >= rules[i].ranges[0][0] && num <= rules[i].ranges[0][1])  || !(num >= rules[i].ranges[1][0] && num <= rules[i].ranges[1][1]) {
				return
			}
		}
	}
	res <- rules
}


func multiplyDepartureFields(conditions []Rule, myTicket []int) {
	res := 1
	for i, condition := range conditions {
		if strings.Contains(condition.description, "departure"){
			res *= myTicket[i]
		}
	}
	fmt.Println("Multiplication of departure values", res)
}


func parseConditions() (bounds []Rule) {
	input, err := ioutil.ReadFile("./16/conditions")
	check(err)

	for _, s := range strings.Split(string(input), "\n"){

		// split so we only have the right part
		s2 := strings.Split(s, ":")

		var l1, h1, l2, h2 int
		fmt.Sscanf(strings.TrimSpace(s2[1]), "%d-%d or %d-%d", &l1, &h1, &l2, &h2)
		bounds = append(bounds, Rule{s2[0], [2][2]int{{l1,h1}, {l2,h2}}})
	}
	return
}
