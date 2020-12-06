package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("./ENTERHERE/input")
	if err != nil {log.Fatalf("ERROR: %s", err)}
	defer file.Close()

	s := bufio.NewScanner(file)
	for s.Scan() { // read input }
	fmt.Printf("result")
}