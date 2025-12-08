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

func main() {
	file, err := os.Open("input.txt")
	// file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		findHighestJoltage(line)
		findHighestJoltage2(line)
	}

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func parseInt(data string) int {
	res, err := strconv.ParseInt(data, 10, 64)
	check(err)
	return int(res)
}

func findHighestJoltage(bank string) {
	highestBattery := 0
	secondHighestBattery := 0

	for i := 0; i < len(bank); i += 1 {
		battery := parseInt(string(bank[i]))
		if battery > highestBattery && i < len(bank)-1 {
			highestBattery = battery
			secondHighestBattery = 0
		} else if battery > secondHighestBattery {
			secondHighestBattery = battery
		}
	}

	combined := fmt.Sprintf("%v%v", highestBattery, secondHighestBattery)
	joltage := parseInt(combined)
	result1 += joltage
}

func findHighestJoltage2(bank string) {
	// First set the number of the resultingBank characters need to be (12)
	// Go through the list finding the highest Number (but only unless 12 characters, including the current one, are left)
	bankToActivate := 0
	// fmt.Println(bank)
	lastHighestidx := 0
	for len(strconv.Itoa(bankToActivate)) < 12 {
		for i := lastHighestidx; i < len(bank)-(12-len(strconv.Itoa(bankToActivate))-1); i += 1 {
			if parseInt(string(bank[i])) > parseInt(string(bank[lastHighestidx])) {
				lastHighestidx = i
			}
		}
		bankToActivate = bankToActivate*10 + parseInt(string(bank[lastHighestidx]))
		lastHighestidx += 1
		// fmt.Println("Bank to activate: ", strconv.Itoa(bankToActivate))
	}

	joltage := bankToActivate
	result2 += joltage
}
