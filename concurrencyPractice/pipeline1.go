/* Explanation:
	Channels can be used to connect goroutines together so that the output of one is the input to another.
	This is called a pipeline.

	|-----------|				|------------|				 |------------|
	|  counter  | (1,2,3)  --> 	|	Squarer  | (1,2,4,9) --> |	 Printer  |
	|-----------|				|------------|				 |------------|
 */

package main

import "fmt"

func main() {
	naturalNumbers := make(chan int)
	squares := make(chan int)

	// Counter
	go func () {
		for x:=0; x < 100; x++ {
			naturalNumbers <- x  // send natural numbers into the naturalNumbers channel
		}
		close(naturalNumbers)
	}()

	go func() {
		for {
			x, ok := <-naturalNumbers // Whenever a number is put into the naturalNumbers Channel it is
			if !ok {
				break // naturalNumbers channel was closed and drained.
			}
			squares <- x * x	// multiply the number you got from the channel and multiply it with itself and send it onto the squares channel.
		}
		close(squares)
	}()

	// Whenever a value is available on the channel take it off and print it.
	for {
		fmt.Println(<-squares)
	}
}
