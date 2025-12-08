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

var result1 = 0
var result2 = 0

func main() {
	// file, err := os.Open("input.txt")
	file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	ranges := strings.Split(line, ",")
	for _, rng := range ranges {
		goThroughRange(rng)
	}

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func goThroughRange(rng string) {
	current, max := parseRange(rng)
	for current <= max {
		if isInvalidId(current) {
			fmt.Println(current, " is invalid 1")
			result1 += int(current)
		}
		if isInvalidId2(current) {
			fmt.Println(current, " is invalid 2")
			result2 += int(current)
		}
		current += 1
	}
}

func parseInt(data string) (int64, error) {
	return strconv.ParseInt(data, 10, 64)
}

func parseRange(rng string) (int64, int64) {
	splitted := strings.Split(rng, "-")
	if len(splitted) != 2 {
		panic("not a valid range")
	}

	min, err := parseInt(splitted[0])
	check(err)
	max, err := parseInt(splitted[1])
	check(err)

	return min, max
}

func isInvalidId(id_i int64) bool {
	id := strconv.FormatInt(id_i, 10)
	// left , right
	return id[:len(id)/2] == id[len(id)/2:]
}

func _isInvalidId2(id string, firstPart string) bool {
	fmt.Println(id, id[:len(firstPart)], firstPart, id[:len(firstPart)] == firstPart, id[len(firstPart):])
	if len(firstPart) == len(id) {
		return id == firstPart
	}

	if id[:len(firstPart)] == firstPart {
		return _isInvalidId2(id[len(firstPart):], firstPart)
	}

	return false
}

func isInvalidId2(id_i int64) bool {
	id := strconv.FormatInt(id_i, 10)

	isInvalid := false
	divider := 2
	fmt.Println("id:", id)
	for divider <= len(id) {
		if math.Mod(float64(len(id)), float64(divider)) == 0 {
			if _isInvalidId2(id[len(id)/divider:], id[:len(id)/divider]) {
				isInvalid = true
			}
		}
		divider += 1
	}

	return isInvalid
}
