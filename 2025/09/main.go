package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var points []Point
var grid [][]Field

var result1 = 0
var result2 = 0

func main() {
	// file, err := os.Open("input.txt")
	file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		processLine(line)
	}

	a, b := findLargestRectangle()
	result1 = a.area(b)

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func processLine(line string) {
	splitted := strings.Split(line, ",")
	x := parseInt(splitted[0])
	y := parseInt(splitted[1])

	points = append(points, Point{x, y})
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

type Field int

const (
	Empty Field = iota
	Green
	Red
)

type Point struct {
	x, y int
}

func (p *Point) area(b Point) int {
	d1 := math.Abs(float64(b.x - p.x + 1))
	d2 := math.Abs(float64(b.y - p.y + 1))
	return int(d1 * d2)
}

func findLargestRectangle() (Point, Point) {
	var p1 Point = points[0]
	var p2 Point = points[1]
	for _, a := range points {
		for _, b := range points {
			if a == b {
				continue
			}

			if a.area(b) > p1.area(p2) {
				p1 = a
				p2 = b
			}
		}
	}
	return p1, p2
}

func getLargestPoints() (Point, Point, Point, Point) {
	topLeft := points[0]
	topRight := points[0]
	bottomLeft := points[0]
	bottomRight := points[0]
	for _, point := range points {
		if point.x < topLeft.x && point.y > topLeft.y {
			topLeft = point
		}
		if point.x > topRight.x && point.y > topRight.y {
			topRight = point
		}
		if point.x < bottomLeft.x && point.y < bottomLeft.y {
			bottomLeft = point
		}
		if point.x > bottomRight.x && point.y < bottomRight.y {
			bottomRight = point
		}
	}

	return topLeft, topRight, bottomLeft, bottomRight
}

func createGrid() {
	_, topRight, _, _ := getLargestPoints()
	grid = make([][]Field, topRight.y+1)
	for _, points := range points {

	}
}
