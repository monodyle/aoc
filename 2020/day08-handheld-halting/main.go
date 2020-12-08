package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.co/monodyle/aoc/helpers"
)

// Instruction represented code
type Instruction struct {
	operation string
	argument  int
}

func main() {
	machine := []Instruction{}
	err := helpers.ScanLines("./input", func(line string) error {
		splitter := strings.Split(line, " ")
		value, _ := strconv.Atoi(splitter[1])
		command := Instruction{
			operation: splitter[0],
			argument:  value,
		}
		machine = append(machine, command)
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	partOne := partOne(machine)
	fmt.Println("Part One:", partOne)
	// fmt.Println("Part Two:", partTwo)
}

func partOne(machine []Instruction) int {
	var acc, i int
	checker := make(map[int]bool, len(machine))
	for {
		if i == len(machine) || checker[i] {
			return acc
		}
		checker[i] = true
		instruct := machine[i]

		switch instruct.operation {
		case "acc":
			acc += instruct.argument
			i++
		case "jmp":
			i += instruct.argument
		case "nop":
			i++
		}
	}
}
