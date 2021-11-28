package main

import (
	"AdventOfCode/util"
	"strings"
)

const (
	activated = true
	deactivated = false
)

type Point [4]int

func (p Point) Add(q Point) (r Point) {
	for i := range p { // xyz(a)
		r[i] = p[i] + q[i]
	}
	return
}

func main() {
	input := util.ReadFile("./17/input") // helper func

	grid := map[Point]bool{} // ('#' = active or '.' = inactive )
	for y, s := range strings.Fields(input){
		for x, c := range s {
			if c == '#' { grid[Point{x, y}] = activated } else if c == '.' {  grid[Point{x,y}] = deactivated }
		}
	}

	// Part 1
	println(act(grid,3))
	// Part 2
	println(act(grid,4)) // 4 dimensions :o
}

func act(grid map[Point]bool, dimensions int) (count int){
	dg := create(dimensions)[1:]

	// fill
	for i := 0; i < 6; i++ {
		for p := range grid {
			for _, d := range dg {
				grid[p.Add(d)] = grid[p.Add(d)]
			}
		}

		new := make(map[Point]bool)
		for p, isActive := range grid {
			activeNeighbors := 0
			for _, d := range dg {
				if grid[p.Add(d)] == activated { activeNeighbors++ }
			}
			if isActive == activated && activeNeighbors == 2 || activeNeighbors == 3 { // check if neighbors are active
				new[p] = activated
			}
		}
		grid = new
	}

	for _, r := range grid {
		if r == activated {
			count++ // res
		}
	}
	return
}

func create(dimensions int) (ds []Point) { // recursive
	if dimensions == 0 { return []Point{{}}} // end of loop

	for _, v := range []	int{0,1,-1} {
		for _, p := range create(dimensions-1){
			p[dimensions-1] = v
			ds = append(ds, p)
		}
	}
	return
}