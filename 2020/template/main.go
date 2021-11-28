package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func check(err error){
	if err != nil {
		panic(err)
	}
}

func main() {
	input, err := ioutil.ReadFile("./num/testinput")
	check(err)

	for _, s := range strings.Split(string(input), "\n"){

	}

	fmt.Println("res", 0)
}