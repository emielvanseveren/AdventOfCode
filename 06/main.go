package main

import (
	"fmt"
	"io/ioutil"
	"strings"
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
	input, _ := ioutil.ReadFile("./06/input")
	c, g:= 0, NewSet() 	// Count, Group set

	for _, l := range strings.Split(string(input), "\n") {
		// A new group is identified by an empty line.
		if l == "" {
			c+= g.Size()
			g.Clear()
			continue
		}

		for i:=0;i<len(l);i++ {
			g.Add(l[i:i+1])
		}
	}
	// Last empty line is not parsed by scanner.
	c += g.Size()
	fmt.Printf("Total count of questions answered by a group (without doubles per question): %d\n", c)
	}

func part2(){
	input,_ := ioutil.ReadFile("./06/input")

	u, g:= NewSet(), NewSet()	// User set, Group set
	c, gc := 0, 0			// Total count, Group count

	for _, l := range strings.Split(string(input), "\n") {

		// A new group is identified by an empty line
		if l == "" {
			c+= g.Size()
			g.Clear()
			gc =0
			continue
		} else {
			gc++
		}

		u.Clear()
		initGroupSize := g.Size()

		for i:=0;i<len(l);i++ {
			if gc == 1 { // First member of group
				g.Add(l[i:i+1])
			} else {
				u.Add(l[i:i+1])
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