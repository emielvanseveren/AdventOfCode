package main

import (
	"fmt"
	"gonum.org/v1/plot"
	"gonum.org/v1/plot/plotter"
	"gonum.org/v1/plot/plotutil"
	"gonum.org/v1/plot/vg"
	"gonum.org/v1/plot/vg/draw"
	"image"
	"image/color"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type Direction string
const (
	north Direction = "N"
	south = "S"
	east = "E"
	west = "W"
	forward = "F"
	left ="L"
	right = "R"
)

type Ship struct  {
	direction Direction
	position image.Point
}

func (s *Ship) GetManhattanDistance() int {
	return int(math.Abs(float64(s.position.X)) + math.Abs(float64(s.position.Y)))
}
func (s *Ship) GetDegrees() int{
	switch s.direction {
	case north:
		return 0
	case east:
		return 90
	case south:
		return 180
	case west:
		return 270
	}
	panic("direction not recognized")
}

func mod(a, b int) int {
	m := a % b
	if a < 0 && b < 0 {
		m -= b
	}
	if a < 0 && b > 0 {
		m += b
	}
	return m
}

func (s *Ship) SetDirection(degrees int) {

	degrees = mod(degrees,360) // Attention standard mod operator does not handle
	fmt.Println(degrees)
	switch degrees {
	case 0:
		s.direction = north
		return
	case 90:
		s.direction = east
		return
	case 180:
		s.direction = south
		return
	case 270:
		s.direction = west
		return
	}
	panic("degrees not found")
}

func (s *Ship) rotate(direction Direction, degrees int){
	if direction == left {
		s.SetDirection(s.GetDegrees() - degrees)
	} else if direction == right {
		s.SetDirection(s.GetDegrees() + degrees)
	}
}
func (s *Ship) Move(direction Direction, amount int) {
	switch direction {
	case north: // 0 -y
		s.position.Y += amount
		break
	case south: // 0 +y
		s.position.Y -= amount
		break
	case east:
		s.position.X += amount
		break
	case west:
		s.position.X -= amount
		break
	case forward:
		// Move forward in the current direction.
		s.Move(s.direction, amount)
		break
	case left:
		s.rotate(direction, amount)
		break
	case right:
		s.rotate(direction, amount)
		break
	}

}

func main() {
	input, _ := ioutil.ReadFile("./12/input")

	p, err := plot.New()
	if err != nil { panic(err)}

	// Props

	// props
	p.Title.Text = "Ship trajectory"
	p.Title.Font.Size = 18
	p.Title.Padding = 0.225*vg.Inch

	p.X.Label.Text="X"
	p.Y.Label.Text="Y"
	p.Add(plotter.NewGrid())

	// line style
	plotter.DefaultLineStyle = draw.LineStyle{ Width: vg.Points(2)}
	colors := []color.Color{ color.RGBA{R: 131, G: 215, B:238, A: 255}}
	plotutil.DefaultColors = colors

	// Start position.
	ship := Ship{ direction: east, position: image.Pt(0,0)} // start position
	pts := make(plotter.XYs, 100)

	for _, i := range strings.Split(strings.TrimSpace(string(input)), "\n"){
		n, _ := strconv.Atoi(i[1:len(i)])
		ship.Move(Direction(i[0:1]),n)
		pts = append(pts, plotter.XY{X: float64(ship.position.X), Y: float64(ship.position.Y)})
	}

	// Connect lines
	err = plotutil.AddLines(p , pts)
	if err != nil { panic(err)}
	// Save plot to png. (if not an err)
	if err := p.Save(10*vg.Inch, 10*vg.Inch, "./12/part1/ship.png"); err != nil { panic(err)}

	// Result
	fmt.Printf("Current position: X: %d, Y: %d \n", ship.position.X, ship.position.Y)
	fmt.Println("Manhattan distance: ", ship.GetManhattanDistance())
}
