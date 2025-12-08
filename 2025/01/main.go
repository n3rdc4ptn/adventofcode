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

const MIN_DIAL_POSITION = 0
const MAX_DIAL_POSITION = 99

var dialPosition int64 = 50

var result1 = 0
var result2 = 0

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		turnDial(line)

		if dialPosition == 0 {
			addResult1("dial=0")
			addResult2("dial=0")
		}
	}

	fmt.Println("Result 1: ", result1)
	fmt.Println("Result 2: ", result2)
}

func parseInt(data string) (int64, error) {
	return strconv.ParseInt(data, 10, 64)
}

func turnDial(data string) {
	fmt.Println(dialPosition, "\tLine: ", data)
	if strings.HasPrefix(data, "L") {
		fmt.Println("\t= L: turning left")
		amount_raw := strings.TrimPrefix(data, "L")
		amount, err := parseInt(amount_raw)
		check(err)

		turnDialLeft(amount)
	}
	if strings.HasPrefix(data, "R") {
		fmt.Println("\t= R: turning right")
		amount_raw := strings.TrimPrefix(data, "R")
		amount, err := parseInt(amount_raw)
		check(err)

		turnDialRight(amount)
	}
	fmt.Println("\t=== new:", dialPosition)
}

// direction (-1 or 1)
func _turnDial(amount int64, direction int) {
	for amount > MAX_DIAL_POSITION {
		amount = amount - MAX_DIAL_POSITION - 1
		addResult2("roation>" + strconv.Itoa((MAX_DIAL_POSITION - 1)))
	}

	newDialPosition := dialPosition + (amount * int64(direction))
	fmt.Println("\tnewDial", newDialPosition, " amount:", (amount * int64(direction)))
	for newDialPosition > MAX_DIAL_POSITION {
		newDialPosition = newDialPosition - MAX_DIAL_POSITION - 1

		if dialPosition != 0 && newDialPosition != 0 {
			addResult2("rotation run over 0")
		}
	}
	for newDialPosition < MIN_DIAL_POSITION {
		newDialPosition = newDialPosition + MAX_DIAL_POSITION + 1

		if dialPosition != 0 && newDialPosition != 0 {
			addResult2("rotation run over 0")
		}
	}
	dialPosition = newDialPosition
}

func turnDialRight(amount int64) {
	_turnDial(amount, 1)
}

func turnDialLeft(amount int64) {
	_turnDial(amount, -1)
}

func addResult1(message string) {
	result1 += 1
	fmt.Println("\t", message, ": result1 + 1 = ", result1)
}

func addResult2(message string) {
	result2 += 1
	fmt.Println("\t", message, ": result2 + 1", result2)
}
