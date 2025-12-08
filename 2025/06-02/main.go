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

type Matrix[T any] [][]T

var result1 = 0
var result2 = 0

var matrix Matrix[string]

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	var lines []string
	width := 0
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
		if width < len(line) {
			width = len(line)
		}
	}
	matrix = newMatrix[string](width, len(lines))
	for y, line := range lines {
		processLine(y, line)
	}

	matrix = matrix.transpose()
	matrix = matrix.flipHorizontally()

	solveTwo(matrix)

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

func processLine(y int, line string) {
	for x, val := range line {
		ch := string(val)
		matrix[y][x] = ch
	}
}

func printMatrix(matrix [][]string) {
	for y := range matrix {
		for x := range matrix[y] {
			fmt.Printf("%v", matrix[y][x])
		}
		fmt.Println()
	}
}

func newMatrix[T any](width, height int) Matrix[T] {
	newMatrix := make(Matrix[T], height)
	for y := 0; y < height; y++ {
		newMatrix[y] = make([]T, width)
	}
	return newMatrix
}

func (m Matrix[T]) height() int {
	return len(m)
}

func (m Matrix[T]) width() int {
	return len(m[0])
}

func (m Matrix[T]) size() (int, int) {
	return m.width(), m.height()
}

func (m Matrix[T]) transpose() Matrix[T] {
	width, height := m.size()
	newMatrix := newMatrix[T](height, width)
	for y := range m {
		for x := range m[y] {
			newMatrix[x][y] = m[y][x]
		}
	}

	return newMatrix
}

func (m Matrix[T]) flipHorizontally() Matrix[T] {
	width, height := m.size()
	newMatrix := newMatrix[T](width, height)
	for y := range m {
		for x := range m[y] {
			newMatrix[height-y-1][x] = m[y][x]
		}
	}

	return newMatrix
}

func solveTwo(matrix Matrix[string]) int {
	problems := make([]MathProblem, 0)
	problem := MathProblem{}
	for _, line := range matrix {
		newString := ""
		for _, char := range line {
			if op := isOperation(char); op != None {
				problem.setOperation(op)
				break
			}
			if char == " " || char == "" {
				continue
			}
			newString += char
		}
		if newString == "" {
			problems = append(problems, problem)
			problem = MathProblem{}
			continue
		}
		val := parseInt(newString)
		problem.addValue(val)
	}
	problems = append(problems, problem)

	for _, problem := range problems {
		result2 += problem.calc()
	}

	return 0
}

func isOperation(val string) MathOperation {
	for k, op := range operations {
		if val == k {
			return op
		}
	}
	return None
}

type MathOperation int

const (
	None MathOperation = iota
	Sum
	Sub
	Div
	Mul
)

var operations map[string]MathOperation = map[string]MathOperation{
	"+": Sum,
	"-": Sub,
	"*": Mul,
	"/": Div,
}

type MathProblem struct {
	operation MathOperation
	values    []int
}

func (p *MathProblem) addValue(value int) {
	p.values = append(p.values, value)
}

func (p *MathProblem) setOperation(operation MathOperation) {
	p.operation = operation
}

func (p *MathProblem) calc() (result int) {
	switch p.operation {
	case Sum:
		for _, val := range p.values {
			result += val
		}
	case Sub:
		for _, val := range p.values {
			result -= val
		}
	case Mul:
		result = p.values[0]
		for _, val := range p.values[1:] {
			result *= val
		}
	case Div:
		result = p.values[0]
		for _, val := range p.values[1:] {
			result /= val
		}
	}

	return result
}
