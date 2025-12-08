package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var result1 = 0
var result2 = 0

const PAPER_ROLL = "@"
const NOTHING = "."

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		loadLineIntoGrid(y, line)
		y++
	}

	checkGrid()

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

var grid [][]string

func loadLineIntoGrid(y int, line string) {
	gridLine := make([]string, len(line))

	for i := 0; i < len(line); i++ {
		char := string(line[i])
		gridLine[i] = char
	}

	if len(grid) > y {
		grid[y] = gridLine
	} else {
		grid = append(grid, gridLine)
	}
}

func isAccessible(x int, y int) bool {
	idzToCheck := [][]int{
		{x - 1, y - 1},
		{x, y - 1},
		{x + 1, y - 1},
		{x - 1, y},
		{x + 1, y},
		{x - 1, y + 1},
		{x, y + 1},
		{x + 1, y + 1},
	}

	numberOfNeighbours := 0
	for _, idz := range idzToCheck {

		x, y := idz[0], idz[1]

		if x < 0 || y < 0 {
			continue
		}
		if x >= len(grid[0]) || y >= len(grid) {
			continue
		}

		if grid[y][x] == PAPER_ROLL {
			numberOfNeighbours++
		}

		if numberOfNeighbours >= 4 {
			return false
		}
	}

	return true
}

func checkGrid() {
	firstRun := true
	rollsToRemove := 0
	for rollsToRemove > 0 || firstRun {
		var dbgGrid [][]string
		rollsToRemove = 0
		for y := range grid {
			gridLine := make([]string, len(grid[y]))
			for x := range grid[y] {
				if grid[y][x] == PAPER_ROLL && isAccessible(x, y) {
					rollsToRemove++
					result2++
					gridLine[x] = "x"
				} else {
					gridLine[x] = grid[y][x]
				}
			}
			dbgGrid = append(dbgGrid, gridLine)
		}
		if firstRun {
			result1 = rollsToRemove
		}

		// printGrid(dbgGrid)
		fmt.Printf("Rolls removed: %v\n", rollsToRemove)
		fmt.Printf("Continue? %v\n", rollsToRemove > 0)
		grid = cleanGrid(dbgGrid)
		firstRun = false
	}
}

func cleanGrid(grid [][]string) [][]string {
	var newGrid [][]string
	for _, line := range grid {
		newLine := make([]string, len(line))
		for x, elem := range line {
			newLine[x] = elem
			if elem == "x" {
				newLine[x] = "."
			}
		}
		newGrid = append(newGrid, newLine)
	}
	return newGrid
}

func printGrid(grid [][]string) {
	fmt.Println("===")
	for y := range grid {
		for x := range grid[y] {
			fmt.Printf("%v ", grid[y][x])
		}
		fmt.Println()
	}
}
