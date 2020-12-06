package main

import (
	"fmt"
	"log"

	"github.co/monodyle/aoc/helpers"
)

const target = 2020

func main() {
	var partOne, partTwo = solution("./input")
	fmt.Println("Part One:", partOne)
	fmt.Println("Part Two:", partTwo)
}

func solution(input string) (int, int) {
	inputs, err := helpers.ParseInputInts(input)
	if err != nil {
		log.Fatal(err)
	}
	return partOne(inputs, target), partTwo(inputs)
}

func partOne(inputs []int, target int) int {
	missing := make(map[int]bool, len(inputs))
	for _, v := range inputs {
		if missing[v] {
			return v * (target - v)
		}
		missing[target-v] = true
	}

	return 0
}

// x + y + z = 2020 => z = 2020 - x - y
func partTwo(inputs []int) int {
	missing := make(map[int]bool, len(inputs))
	for i, x := range inputs {
		for _, y := range inputs[i+1:] {
			z := target - x - y
			if missing[z] {
				return x * y * z
			}
		}
		missing[x] = true
	}
	return 0
}
