package main

import (
	"fmt"
	"image"
	/* Image contains a POINT (x,y) great for grid usage.
	Which also has handy vector functions such as add, mul...
	https://golang.org/pkg/image/#Point */
	"io/ioutil"
	"strings"
)

// Interface for func for different condition in part1/part2
type adjacent func(image.Point, image.Point) image.Point

func main() {
	input, _ := ioutil.ReadFile("./11/input")

	seats := map[image.Point]rune{}
	for i, s := range strings.Split(strings.TrimSpace(string(input)), "\n") {
		for j, r := range s {
			seats[image.Pt(j, i)] = r
		}
	}

	part1 := head(seats, func(p, d image.Point) image.Point { return p.Add(d) }, 4)
	fmt.Printf("Occupied seats: %d", part1)

	part2 := head(seats, func(p, d image.Point) image.Point {
		for seats[p.Add(d)] == '.' {
			p = p.Add(d)
		}
		return p.Add(d)
	}, 5)
	fmt.Printf("Occupied seats: %d", part2)
}

func head(seats map[image.Point]rune, adjacent adjacent, maxAdj int) (occupied int) {
	directions := []image.Point{
		{0, -1}, 		// TOP-CENTER
		{1, -1},		// TOP-RIGHT
		{1, 0},		// RIGHT
		{1, 1},		// BOTTOM-RIGHT
		{-1, -1},		// BOTTOM-LEFT
		{0, 1},		// BOTTOM-CENTER
		{-1, 0},		// LEFT
		{-1, 1},		// TOP-LEFT
	}
	diff := true
	for  diff {
		// reset
		occupied = 0
		diff = false

		next:= map[image.Point]rune{}
		for p, r := range seats {
			sum := 0
			for _, d := range directions {

				if seats[adjacent(p, d)] == '#' {
					sum++
				}
			}

			// set to empty.
			if r == '#' && sum >= maxAdj {
				r = 'L'
			} else if r == 'L' && sum == 0 || r == '#' {
				r = '#'
				occupied++
			}
			next[p] = r
			diff = diff || next[p] != seats[p] // should go till no more changes.
		}
		seats = next //
	}
	return
}
