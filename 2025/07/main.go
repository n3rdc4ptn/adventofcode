package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var result1 = 0
var result2 = 0

var grid Grid

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parseLine(line)
	}

	fmt.Printf("==========\n")
	for _, line := range grid.data {
		for _, elem := range line {
			fmt.Printf("%v ", elem.print())
		}
		fmt.Println()
	}
	fmt.Printf("==========\n")

	for !grid.isEnd() {
		grid.oneStep()
	}

	result2 += grid.weightSum()

	fmt.Println("Result1: ", result1)

	fmt.Println("Result2: ", result2)
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

type Element int

const (
	Empty Element = iota
	Splitter
	StartPoint
)

var elements map[string]Element = map[string]Element{
	".": Empty,
	"S": StartPoint,
	"^": Splitter,
}

func (e Element) print() string {
	switch e {
	case Empty:
		return "."
	case StartPoint:
		return "S"
	case Splitter:
		return "^"
	default:
		return ""
	}
}

func parseElement(input string) Element {
	return elements[input]
}

func parseLine(line string) {
	newLine := make([]Element, len(line))
	shouldBeStored := false
	for idx, c := range line {
		ch := string(c)
		newLine[idx] = parseElement(ch)
		shouldBeStored = true
		// if newLine[idx] != Empty {
		// }
	}
	if shouldBeStored {
		grid.data = append(grid.data, newLine)
	}
}

type Point struct {
	x      int
	y      int
	weight int
}

func (p *Point) string() string {
	return fmt.Sprintf("[%v,%v]", p.x, p.y)
}

type Grid struct {
	data     [][]Element
	tachyons []Point
}

func (g *Grid) startingPoint() Point {
	for x, elem := range g.data[0] {
		if elem == StartPoint {
			return Point{
				x:      x,
				y:      0,
				weight: 1,
			}
		}
	}
	return Point{}
}

func (g *Grid) get(x, y int) Element {
	return g.data[y][x]
}
func (g *Grid) getByPoint(p Point) Element {
	return g.get(p.x, p.y)
}

func (g *Grid) isEnd() bool {
	if len(g.tachyons) == 0 {
		return false
	}
	return g.tachyons[0].y == len(g.data)-1
}

func indexOf(t []Point, p Point) int {
	return slices.IndexFunc(t, func(el Point) bool {
		return p.x == el.x && p.y == el.y
	})
}

func (g *Grid) oneStep() {
	if len(g.tachyons) == 0 {
		g.tachyons = append(g.tachyons, g.startingPoint())
	}
	newTachyons := make([]Point, 0)

	for _, tachyon := range g.tachyons {
		tachyon.y++
		if g.getByPoint(tachyon) == Splitter {
			left := Point{x: tachyon.x - 1, y: tachyon.y, weight: tachyon.weight}
			right := Point{x: tachyon.x + 1, y: tachyon.y, weight: tachyon.weight}

			if idx := indexOf(newTachyons, left); idx != -1 {
				newTachyons[idx].weight += left.weight
			} else {
				newTachyons = append(newTachyons, left)
			}
			if idx := indexOf(newTachyons, right); idx != -1 {
				newTachyons[idx].weight += right.weight
			} else {
				newTachyons = append(newTachyons, right)
			}
			result1++
		} else {
			if idx := indexOf(newTachyons, tachyon); idx != -1 {
				newTachyons[idx].weight += tachyon.weight
			} else {
				newTachyons = append(newTachyons, tachyon)
			}
		}
	}

	g.tachyons = newTachyons
}

func (g *Grid) weightSum() int {
	s := 0
	for _, t := range g.tachyons {
		if t.y == len(g.data)-1 {
			s += t.weight
		}
	}
	return s
}
