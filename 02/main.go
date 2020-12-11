package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("./02/input")

	if err != nil {
		log.Fatal("ERROR: %s", err)
	}
	// this will defer the execution until the surrounding function returns (main)
	defer file.Close()

	/* part 1 */
	corr:=0
	corr2 :=0

	s:= bufio.NewScanner(file)
	for s.Scan() {
		o := strings.Split(s.Text(), ":") // [condition, password]
		if passwordIsValid(o[0], strings.TrimSpace(o[1])) {
			corr++;
		}
		if passwordIsValid2(o[0], strings.TrimSpace(o[1])) {
			corr2++;
		}
	}
	fmt.Printf("Amount of correct passwords for part 1: %d \n", corr)
	fmt.Printf("Amount of correct passwords for part 2: %d \n", corr2)
}

func passwordIsValid(condition string, password string) bool {

	o := strings.Split(condition, " ") // [length, letter]
	length, conditionLetter := o[0], o[1]

	l := strings.Split(length, "-") // [min, max]
	min,minErr := strconv.Atoi(l[0])
	max, maxErr :=strconv.Atoi(l[1])

	if minErr != nil || maxErr != nil {
		log.Fatal("var min or max could not be parsed to an integer")
	}

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
	pos,posErr:= strconv.Atoi(l[0])
	pos2, pos2Err:=strconv.Atoi(l[1])

	if posErr != nil || pos2Err!= nil {
		log.Fatal("Could not extract positions(int) from given string");
	}


	// *pos1* should contain *conditionletter* or *pos2* should contain *conditionletter*. BUT NOT BOTH.
	if conditionLetter == password[pos-1:pos] && conditionLetter != password[pos2-1:pos2] || conditionLetter == password[pos2-1:pos2] && conditionLetter != password[pos-1:pos] {
		fmt.Printf("true \n")
		return true
	}
	return false
}