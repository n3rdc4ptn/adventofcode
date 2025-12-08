package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var result1 = 0
var result2 = 0

var problems []MathProblem

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	columns := strings.Split(line, " ")
	problems = make([]MathProblem, len(columns))
	processLine(line)
	for scanner.Scan() {
		line := scanner.Text()
		processLine(line)
	}

	for _, problem := range problems {
		result1 += problem.calc()
	}

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

func processLine(line string) {
	numbers := strings.Split(line, " ")
	idx := 0
	for _, val := range numbers {
		if val == "" {
			continue
		}
		if op := isOperation(val); op != None {
			problems[idx].setOperation(op)
		} else {
			n := parseInt(val)

			problems[idx].addValue(n)
		}
		idx++
	}
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
