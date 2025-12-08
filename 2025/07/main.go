package main

import (
	"bufio"
	"fmt"
	"os"
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

	// fmt.Printf("==========\n")
	// for _, line := range grid.data {
	// 	for _, elem := range line {
	// 		fmt.Printf("%v ", elem.print())
	// 	}
	// 	fmt.Println()
	// }
	// fmt.Printf("==========\n")

	for !grid.isEnd() {
		grid.oneStep()
	}

	fmt.Println("Result1: ", result1)

	result2 = grid.timelineCheck(grid.startingPoint())

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
		if newLine[idx] != Empty {
			shouldBeStored = true
		}
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

type Grid struct {
	data      [][]Element
	tachyons  []Point
	tachyons2 []Point
}

func (g *Grid) startingPoint() Point {
	for x, elem := range g.data[0] {
		if elem == StartPoint {
			return Point{
				x: x,
				y: 0,
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

func (g *Grid) isEnd2() bool {
	if len(g.tachyons2) == 0 {
		return false
	}
	return g.tachyons2[0].y == len(g.data)-1
}

func (g *Grid) oneStep() {
	if len(g.tachyons) == 0 {
		g.tachyons = append(g.tachyons, g.startingPoint())
	}
	t_map := make(map[Point]bool, 0)
	newTachyons := make([]Point, 0)

	for _, tachyon := range g.tachyons {
		tachyon.y++
		if g.getByPoint(tachyon) == Splitter {
			left := Point{x: tachyon.x - 1, y: tachyon.y}
			right := Point{x: tachyon.x + 1, y: tachyon.y}
			if _, ok := t_map[left]; !ok {
				t_map[left] = true
				newTachyons = append(newTachyons, left)
			}
			if _, ok := t_map[right]; !ok {
				t_map[right] = true
				newTachyons = append(newTachyons, right)
			}
			result1++
		} else {
			if _, ok := t_map[tachyon]; !ok {
				t_map[tachyon] = true
				newTachyons = append(newTachyons, tachyon)
			}
		}
	}

	g.tachyons = newTachyons
}
func (g *Grid) oneStep2() {
	if len(g.tachyons2) == 0 {
		g.tachyons2 = append(g.tachyons2, g.startingPoint())
	}
	newTachyons := make([]Point, 0)

	for _, tachyon := range g.tachyons2 {
		tachyon.y++
		if g.getByPoint(tachyon) == Splitter {
			left := Point{x: tachyon.x - 1, y: tachyon.y}
			right := Point{x: tachyon.x + 1, y: tachyon.y}
			newTachyons = append(newTachyons, left)
			newTachyons = append(newTachyons, right)
		} else {
			newTachyons = append(newTachyons, tachyon)
		}
	}

	g.tachyons2 = newTachyons
}

func (g *Grid) timelineCheck(tachyon Point) int {
	// if tachyon.y%2 == 0 {
	// 	fmt.Printf("%v%%\n", int(float64(tachyon.y)/float64(len(g.data)-1)*100))
	// }
	if tachyon.y >= len(g.data)-1 {
		return 1
	}
	tachyon.y++

	if g.getByPoint(tachyon) == Splitter {
		return g.timelineCheck(Point{
			x: tachyon.x - 1,
			y: tachyon.y,
		}) + g.timelineCheck(Point{
			x: tachyon.x + 1,
			y: tachyon.y,
		})
	}

	return g.timelineCheck(tachyon)
}
