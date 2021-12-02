package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.co/monodyle/aoc/2020/helpers"
)

// Instruction represented code
type Instruction struct {
	operation string
	argument  int
}

func main() {
	machine := []Instruction{}
	err := helpers.ScanLines("./inputTest", func(line string) error {
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

	partOne, _ := partOne(machine)
	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partTwo(machine))
}

func partOne(machine []Instruction) (int, bool) {
	var acc, i int
	checker := make(map[int]bool, len(machine))
	for {
		if i == len(machine) {
			return acc, true
		}
		if checker[i] {
			return acc, false
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

func partTwo(machine []Instruction) int {
	for i, instruct := range machine {
		var faker []Instruction
		if instruct.operation == "nop" {
			faker = copyCat(machine)
			faker[i].operation = "jmp"
		} else if instruct.operation == "jmp" {
			faker = copyCat(machine)
			faker[i].operation = "nop"
		}

		if faker != nil {
			acc, complete := partOne(faker)
			if complete {
				return acc
			}
		}
	}
	return 0
}

func copyCat(machine []Instruction) []Instruction {
	factory := make([]Instruction, len(machine))
	copy(factory, machine)
	return factory
}
