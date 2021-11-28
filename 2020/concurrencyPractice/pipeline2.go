/*
 Our previous example uses 3 go routines (the main func is also a go routine). communicating over 2 channels.
	As the program grows we divide the program in smaller pieces

	func counter(out chan int)
	func squarer(out, in chan int)
	func printer(in chan int)

	The names in and out convey that they are intended to send and receive. But nothing prevents squarer from sending to
	in or receiving from out. The go type system provides unidirection channel types that expose only one or the other
	of the send or receive operations.

 */

package main

import "fmt"

func counter(out chan<- int){ // we put something into the channel
	for x:=0; x<100;x++{
		out <- x
	}
	close(out)
}
func squarer(in <-chan int, out chan<- int){ // (in = input, for this function, out = output for this func)
	for v:= range in {
		out <- v*v
	}
	close(out)
}

func printer(in <- chan int){
	for v:= range in {
		fmt.Println(v)
	}
}

func main() {
	naturalNumbers := make(chan int)
	squares := make(chan int)

	go counter(naturalNumbers)
	go squarer(naturalNumbers, squares)
	printer(squares)
}

/*
	Since the close operation asserts that no more sends will occur on a channel, only the sending goroutine is in a
	position to call it, and for this reason it is a compile-time error to attempt to close a receive-only channel.
 */
