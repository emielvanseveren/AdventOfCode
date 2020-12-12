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
	position image.Point
	wayPoint image.Point
}

func (s *Ship) GetManhattanDistance() int {
	return int(math.Abs(float64(s.position.X)) + math.Abs(float64(s.position.Y)))
}

func (s *Ship) rotate(direction Direction, degrees int){
	if direction == left {
		for i:=0; i<degrees/90;i++{
			s.wayPoint.X, s.wayPoint.Y = -s.wayPoint.Y, s.wayPoint.X /* thx :) @github.com/niekcandaele */
		}
	} else if direction == right {
		for i:=0; i<degrees/90;i++{
			s.wayPoint.X, s.wayPoint.Y  = s.wayPoint.Y, -s.wayPoint.X /* also thx :) */
		}
	}
}
func (s *Ship) Move(direction Direction, amount int) {
	switch direction {
	case north: // 0 -y
		s.wayPoint.Y += amount
		break
	case south: // 0 +y
		s.wayPoint.Y -= amount
		break
	case east:
		s.wayPoint.X += amount
		break
	case west:
		s.wayPoint.X -= amount
		break
	case forward:
		s.position.X += amount * s.wayPoint.X
		s.position.Y += amount * s.wayPoint.Y
		break
	case left, right:
		s.rotate(direction, amount)
		break
	}
}

func main() {
	input, _ := ioutil.ReadFile("./12/input")

	p, err := plot.New()
	if err != nil { panic(err)}

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

	// Start position
	ship := Ship{ position: image.Pt(0,0), wayPoint: image.Pt(10, 1)}
	pts := make(plotter.XYs, 100)
	for _, i := range strings.Split(strings.TrimSpace(string(input)), "\n"){
		n, _ := strconv.Atoi(i[1:len(i)])
		ship.Move(Direction(i[0:1]),n)
		pts = append(pts, plotter.XY{X: float64(ship.position.X), Y: float64(ship.position.Y)})
	}

	// Connect points with lines

	err = plotutil.AddLines(p , pts)

	if err != nil { panic(err)}
	// Save plot to png. (if not an err)
	if err := p.Save(10*vg.Inch, 10*vg.Inch, "./12/part2/ship.png"); err != nil { panic(err)}

	// Result
	fmt.Printf("Current position: X: %d, Y: %d \n", ship.position.X, ship.position.Y)
	fmt.Println("Manhattan distance: ", ship.GetManhattanDistance())
}
