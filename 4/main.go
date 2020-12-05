package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

type passport struct {
	birthYear 			int
	issueYear 			int
	expirationYear 		int
	height				string
	hairColor			string
	eyeColor			string
	passportId			int
}

func (p *passport) IsValid() bool {
	return false
}

func main() {
	file, err := os.Open("./4/input")

	if err != nil {
		log.Fatal(err)
	}

	s := bufio.NewScanner(file)

	// passports:= // array of passports

	for s.Scan() {

		if s.Text() != "" {
			keyWithValue := strings.Split(s.Text(), " ")
			// for key := ra
			d := strings.Split(keyWithValue, ":")
	} else {
		}

	}
}
