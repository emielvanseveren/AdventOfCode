package main

import (
	"fmt"
	"time"
)

/*
In the example below; the main goroutine computes the 45th Fibonacci number. Since it uses the terribly inefficient
recursive algorithm, it runs for an appreciable time, during which we'd like to provide the user with a visual
indication that the program is still running, by displaying an animated textual "spinner"
 */

func main() {
	go spinner(100*time.Millisecond)
	const n = 45
	fmt.Printf("\rFibonnaci(%d) = %d\n", n, fib(n))

}
func fib(x int) int{
	if x < 2 {
		return x
	}
	return fib(x-1) + fib(x-2)
}
func spinner(delay time.Duration){
	for _, r:= range `-\|/` {
		fmt.Printf("\r%c", r)
		time.Sleep(delay)
	}
}
