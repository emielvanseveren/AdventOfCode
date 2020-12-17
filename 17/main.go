package main

import (
	"AdventOfCode/util"
	"strings"
)

type Point [4]int

func (p Point) Add(q Point) (r Point) {
	for i := range p {
		r[i] = p[i] + q[i]
	}
	return
}

func main() {
	input := util.ReadFile("./17/input") // helper func

	grid := map[Point]rune{} // ('#' or '.')
	for y, s := range strings.Fields(input){
		for x, c := range s {
			grid[Point{x, y}] = c
		}
	}
	println(run(grid,3))
	println(run(grid,4))
}

func run(grid map[Point]rune, dimensions int) (count int){
	dg := create(dimensions)[1:]


	// fill
	for i := 0; i < 6; i++ {
		for p := range grid {
			for _, d := range dg {
				grid[p.Add(d)] = grid[p.Add(d)]
			}
		}

		new := make(map[Point]rune)
		for p, r := range grid {
			neigh := 0
			for _, d := range dg {
				if grid[p.Add(d)] == '#' {
					neigh++
				}
			}

			if r == '#' && neigh == 2 || neigh == 3 {
				new[p] = '#'
			}
		}
		grid = new
	}

	// count result
	for _, r := range grid {
		if r == '#' {
			count++
		}
	}
	return
}

func create(dimensions int) (ds []Point) { // recursive
	if dimensions == 0 {
		return []Point{{}}
	}

	for _, v := range []	int{0,1,-1} {
		for _, p := range create(dimensions-1){
			p[dimensions-1] = v
			ds = append(ds, p)
		}
	}
	return
}