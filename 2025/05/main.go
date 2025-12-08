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

var db [][]int
var db2 map[string]Range = make(map[string]Range)

var freshIds map[int]bool = make(map[int]bool)

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		processRange(line)
	}

	fmt.Println(db2)
	cleanUpRanges()
	fmt.Println(db2)

	result2 = coundIds()

	for scanner.Scan() {
		line := scanner.Text()
		if isFresh(line) {
			result1++
		}
	}

	fmt.Println("Len Fresh Ids:", len(freshIds))

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

func processRange(line string) {
	splitted := strings.Split(line, "-")
	if len(splitted) != 2 {
		panic("Unknown range " + line)
	}

	min := parseInt(splitted[0])
	max := parseInt(splitted[1])

	db = append(db, []int{min, max})
	if _, ok := db2[line]; !ok {
		db2[line] = newRange(min, max)
	}
}

func isFresh(inputId string) bool {
	id := parseInt(inputId)

	for _, rng := range db {
		if id >= rng[0] && id <= rng[1] {
			return true
		}
	}

	return false
}

type Range struct {
	min int
	max int
}

func newRange(min, max int) Range {
	if min > max {
		return Range{
			min: max,
			max: min,
		}
	}
	return Range{
		min,
		max,
	}
}

func (r *Range) string() string {
	return fmt.Sprintf("%v-%v", r.min, r.max)
}

// a-b, c-d
// return a list of new ranges
func compareRanges(a, b Range) []Range {
	// inside: b in a
	if a.min <= b.min && b.max <= a.max {
		return []Range{a}
	}
	// inside: a in b
	if b.min <= a.min && a.max <= b.max {
		return []Range{b}
	}

	// overlap: a b a b
	if a.min <= b.min && b.min <= a.max {
		return []Range{newRange(a.min, b.max)}
	}
	// overlap: b a b a
	if b.min <= a.min && a.min <= b.max {
		return []Range{newRange(b.min, a.max)}
	}

	// beneath each other
	return []Range{a, b}
}

func cleanUpRanges() {
	isClean := false
	for !isClean {
		isClean = true
		var newDb map[string]Range = make(map[string]Range)
		for _, a := range db2 {
			for _, b := range newDb {
				delete(newDb, a.string())
				delete(newDb, b.string())
				// fmt.Printf("Compare %v with %v\t", a, b)
				newRanges := compareRanges(a, b)
				// fmt.Printf("Result %v\t\t", newRanges)
				for _, c := range newRanges {
					if _, ok := newDb[c.string()]; !ok {
						newDb[c.string()] = c
					}
				}
				// fmt.Printf("NewDB: %v\n", newDb)
				if len(newRanges) != 2 {
					isClean = false
				}
			}
			if len(newDb) == 0 {
				newDb[a.string()] = a
			}
		}
		db2 = newDb
	}
}

func coundIds() int {
	amount := 0
	for _, a := range db2 {
		amount += a.max - a.min + 1
	}
	return amount
}
