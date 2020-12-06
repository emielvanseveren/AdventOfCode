package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Set struct {
	list map[string]struct{} // Empty structs occupies 0 memory
}
func (s *Set) Has(v string) bool {
	_, ok := s.list[v]
	return ok
}
func (s *Set) Add(v string) {
	s.list[v] = struct{}{}
}
func (s *Set) Clear() {
	s.list = make(map[string]struct{})
}
func (s *Set) Size() int {
	return len(s.list)
}
func NewSet() *Set {
	s := &Set{}
	s.list = make(map[string]struct{})
	return s
}
func (s *Set) Intersect(s2 *Set) *Set {
	res := NewSet()
	for v := range s.list {
		if s2.Has(v) == false {
			continue
		}
		res.Add(v)
	}
	return res
}

func part1(){
	file, err := os.Open("./6/input")

	if err != nil {log.Fatal("ERROR: %s", err)}
	defer file.Close()

	s := bufio.NewScanner(file)
	g:= NewSet() 	// Group set
	c:=0 			// Total count

	for s.Scan() {
		line := s.Text()

		// A new group is identified by an empty line.
		if line == "" {
			c+= g.Size()
			g.Clear()
			continue
		}

		for i:=0;i<len(line);i++ {
			g.Add(line[i:i+1])
		}
	}
	// Last empty line is not parsed by scanner.
	c += g.Size()
	fmt.Printf("Total count of questions answered by a group (without doubles per question): %d \n", c)
}

func part2(){
	file, err := os.Open("./6/input")
	if err != nil {log.Fatal("ERROR: %s", err)}
	defer file.Close()

	s := bufio.NewScanner(file)
	g:= NewSet()	// Group set
	u:= NewSet()	// User set
	c:=0			// Total count
	gc:= 0 			// Group count

	for s.Scan() {
		line := s.Text()

		// A new group is identified by an empty line
		if line == "" {
			c+= g.Size()
			g.Clear()
			gc =0
			continue
		} else {
			gc++
		}

		u.Clear()
		initGroupSize := g.Size()

		for i:=0;i<len(line);i++ {
			if gc == 1 { // First member of group
				g.Add(line[i:i+1])
			} else {
				u.Add(line[i:i+1])
			}
		}
		if initGroupSize != 0 {
			g= g.Intersect(u)
		}
	}
	c+= g.Size() // Last empty line is not parsed by scanner.
	fmt.Printf("Total question count of question that are answered by the entire group: %d \n", c)
}

func main() {
	part1()
	part2()
}