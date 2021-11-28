package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {

	input, _:= ioutil.ReadFile("./02/input")

	c1:=0
	c2:=0

	for _,s := range strings.Split(string(input),"\n"){
		o := strings.Split(s, ":") // [condition, password]
		if passwordIsValid(o[0], strings.TrimSpace(o[1])) { c1++ } 	// part 1
		if passwordIsValid2(o[0], strings.TrimSpace(o[1])) { c2++ } // part 2
	}
	fmt.Println("Amount of correct passwords for part 1:", c1)
	fmt.Println("Amount of correct passwords for part 2:", c2)
}

func passwordIsValid(condition string, password string) bool {

	o := strings.Split(condition, " ") // [length, letter]
	length, conditionLetter := o[0], o[1]

	l := strings.Split(length, "-") // [min, max]
	min,_:= strconv.Atoi(l[0])
	max, _:=strconv.Atoi(l[1])

	count :=0
	for i:=0;i<len(password);i++ {
		if conditionLetter == password[i:i+1] {
			count++
		}
	}

	if count >= min && count <= max {
		return true
	}
	return false
}

func passwordIsValid2(condition string, password string) bool {
	o := strings.Split(condition, " ") // [length, letter]
	length, conditionLetter := o[0], o[1]

	l := strings.Split(length, "-") // [min, max]
	pos, _:= strconv.Atoi(l[0])
	pos2, _:=strconv.Atoi(l[1])

	// *pos1* should contain *conditionletter* or *pos2* should contain *conditionletter*. BUT NOT BOTH.
	if conditionLetter == password[pos-1:pos] && conditionLetter != password[pos2-1:pos2] || conditionLetter == password[pos2-1:pos2] && conditionLetter != password[pos-1:pos] {
		return true
	}
	return false
}