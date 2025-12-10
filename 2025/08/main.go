package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var data []Point

var result1 = 0
var result2 = 0

var CONNECTIONS_LEFT = 10

// var CONNECTIONS_LEFT = 1000

func main() {
	// file, err := os.Open("input.txt")
	file, err := os.Open("example_input.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		processLine(line)
	}

	pairs := generatePairs()
	slices.SortFunc(pairs, ComparePairs)
	_ = connectPairs2(pairs, CONNECTIONS_LEFT)

	circuits := connectPairs(pairs, CONNECTIONS_LEFT)
	fmt.Println(circuits)
	slices.SortFunc(circuits, func(a, b Circuit) int {
		return b.len() - a.len()
	})

	// fmt.Println(circuits)

	result1 = 1
	for _, circuit := range circuits[:3] {
		result1 *= circuit.len()
	}

	fmt.Println("Result1: ", result1)
	fmt.Println("Result2: ", result2)
}

func processLine(line string) {
	splitted := strings.Split(line, ",")
	if len(splitted) != 3 {
		return
	}

	data = append(data, newPoint(parseInt(splitted[0]), parseInt(splitted[1]), parseInt(splitted[2])))
}

func parseInt(raw string) int {
	num, err := strconv.ParseInt(raw, 10, 64)
	check(err)

	return int(num)
}

type Point struct {
	x int
	y int
	z int
}

func newPoint(x, y, z int) Point {
	return Point{
		x: x,
		y: y,
		z: z,
	}
}

func (p *Point) id() string {
	return fmt.Sprintf("[%v,%v,%v]", p.x, p.y, p.z)
}

func (p *Point) Equal(b Point) bool {
	return p.x == b.x && p.y == b.y && p.z == b.z
}

func (p *Point) distance(p2 Point) float64 {
	x_d := float64(p.x - p2.x)
	y_d := float64(p.y - p2.y)
	z_d := float64(p.z - p2.z)
	return math.Round(math.Sqrt(math.Pow(x_d, 2)+math.Pow(y_d, 2)+math.Pow(z_d, 2))*100) / 100
}

func (p *Point) distanceInt(p2 Point) int {
	x_d := float64(p.x - p2.x)
	y_d := float64(p.y - p2.y)
	z_d := float64(p.z - p2.z)
	return int(math.Sqrt(math.Pow(x_d, 2) + math.Pow(y_d, 2) + math.Pow(z_d, 2)))
}

type Pair struct {
	a int
	b int
}

func (p *Pair) id() string {
	if p.a < p.b {
		return fmt.Sprintf("%v-%v", p.a, p.b)
	}
	return fmt.Sprintf("%v-%v", p.b, p.a)
}

func newPair(a, b int) Pair {
	return Pair{
		a, b,
	}
}

func (p *Pair) Equal(b Pair) bool {
	return (p.a == b.a && p.b == b.b) || (p.a == b.b && p.b == b.a)
}

func ComparePairs(a, b Pair) int {
	return int(a.distance() - b.distance())
}

func (p *Pair) distance() float64 {
	return data[p.a].distance(data[p.b])
}

type Circuit struct {
	points []int
}

func newCircuit(p []int) Circuit {
	return Circuit{points: p}
}

func newCircuitFromPair(p Pair) Circuit {
	return newCircuit([]int{
		p.a, p.b,
	})
}

func (c *Circuit) add(p int) {
	c.points = append(c.points, p)
}

func (c *Circuit) len() int {
	return len(c.points)
}

func (c *Circuit) contains(p int) bool {
	return slices.Contains(c.points, p)
}

func (c *Circuit) distance(cb Circuit) float64 {
	var shortest_distance float64 = -1
	for _, p := range c.points {
		for _, pb := range cb.points {
			distance := data[p].distance(data[pb])
			if distance < shortest_distance || shortest_distance == -1 {
				shortest_distance = distance
			}
		}
	}
	return shortest_distance
}

func (c *Circuit) addUnique(p int) bool {
	contains := c.contains(p)
	if !contains {
		c.add(p)
	}

	return !contains
}

func printCircuits(circuits []Circuit) {
	fmt.Printf("[")
	for _, circuit := range circuits {
		fmt.Printf("%v ", circuit.points)
	}
	fmt.Println("]")
}

func generatePairs() []Pair {
	result := make([]Pair, 0)
	m := make(map[string]bool, 0)
	for a := range data {
		for b := range data {
			if data[a].Equal(data[b]) {
				continue
			}

			pair := newPair(a, b)
			if _, ok := m[pair.id()]; !ok {
				result = append(result, pair)
				m[pair.id()] = true
			}
		}
	}

	return result
}

// Pairs are connected in the order they are listed. From top to bottom.
func connectPairs(pairs []Pair, max_connections int) []Circuit {
	circuits := make([]Circuit, 0, len(pairs))
	for _, pair := range pairs[:max_connections] {
		// fmt.Printf("%v\n", idx)
		// fmt.Printf("%v: ", CONNECTIONS_LEFT-max_connections)
		var circuit_a int = -1
		var circuit_b int = -1
		for idx, circuit := range circuits {
			// first check if a circuit already contains one of the pairs elements
			if circuit.contains(pair.a) {
				circuit_a = idx
			}
			if circuit.contains(pair.b) {
				circuit_b = idx
			}
		}
		// printCircuits(circuits)

		if circuit_a == -1 && circuit_b == -1 {
			// fmt.Printf("Creating new circuit for pair: %v\n", pair)
			// no one is in any circuit, create new one
			circuits = append(circuits, newCircuitFromPair(pair))
		} else if circuit_a == circuit_b {
			// fmt.Printf("Same circuit, nothing happens: %v\n", pair)
			// same circuit, should not happen, skip it
		} else if circuit_a == -1 {
			// fmt.Printf("%v is in circuit %v. Adding %v to it\n", pair.b, circuits[circuit_b], pair.a)
			// a isn't in any circuit, add it so b's circuit
			circuits[circuit_b].add(pair.a)
		} else if circuit_b == -1 {
			// fmt.Printf("%v is in circuit %v. Adding %v to it\n", pair.a, circuits[circuit_a], pair.b)
			// a isn't in any circuit, add it so b's circuit
			circuits[circuit_a].add(pair.b)
		} else {
			// Two circuits, connect them
			circuit := circuits[circuit_a]
			circuit.points = append(circuit.points, circuits[circuit_b].points...)
			circuits[circuit_a].points = []int{}
			circuits[circuit_b].points = []int{}
			circuits = append(circuits, circuit)
		}
	}

	return circuits
}

func isOnlyOneCircuit(circuits []Circuit) bool {
	isOnlyOne := false
	for _, circuit := range circuits {
		if circuit.len() != 0 && isOnlyOne {
			return false
		}
		if circuit.len() != 0 && !isOnlyOne {
			isOnlyOne = true
		}
	}

	return isOnlyOne
}

func connectPairs2(pairs []Pair, max_connections int) []Circuit {
	circuits := make([]Circuit, 0, len(pairs))
	for idx, pair := range pairs {
		fmt.Printf("%v\n", idx)
		// fmt.Printf("%v: ", CONNECTIONS_LEFT-max_connections)
		var circuit_a int = -1
		var circuit_b int = -1
		for idx, circuit := range circuits {
			// first check if a circuit already contains one of the pairs elements
			if circuit.contains(pair.a) {
				circuit_a = idx
			}
			if circuit.contains(pair.b) {
				circuit_b = idx
			}
		}
		// printCircuits(circuits)

		if circuit_a == -1 && circuit_b == -1 {
			// fmt.Printf("Creating new circuit for pair: %v\n", pair)
			// no one is in any circuit, create new one
			circuits = append(circuits, newCircuitFromPair(pair))
		} else if circuit_a == circuit_b {
			// fmt.Printf("Same circuit, nothing happens: %v\n", pair)
			// same circuit, should not happen, skip it
		} else if circuit_a == -1 {
			// fmt.Printf("%v is in circuit %v. Adding %v to it\n", pair.b, circuits[circuit_b], pair.a)
			// a isn't in any circuit, add it so b's circuit
			circuits[circuit_b].add(pair.a)
		} else if circuit_b == -1 {
			// fmt.Printf("%v is in circuit %v. Adding %v to it\n", pair.a, circuits[circuit_a], pair.b)
			// a isn't in any circuit, add it so b's circuit
			circuits[circuit_a].add(pair.b)
		} else {
			// Two circuits, connect them
			circuit := circuits[circuit_a]
			circuit.points = append(circuit.points, circuits[circuit_b].points...)
			circuits[circuit_a].points = []int{}
			circuits[circuit_b].points = []int{}
			circuits = append(circuits, circuit)
		}
		max_connections--

		if max_connections <= 0 && isOnlyOneCircuit(circuits) {
			result2 = data[pair.a].x * data[pair.b].x
			fmt.Println(data[pair.a].x, data[pair.b].x)
			break
		}
	}

	return circuits
}
