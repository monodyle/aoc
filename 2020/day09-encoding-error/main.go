package main

import (
	"fmt"
	"log"

	"github.co/monodyle/aoc/helpers"
)

func main() {
	inputs, err := helpers.ParseInputInts("./input")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Part 1:", partOne(inputs))
}

func partOne(inputs []int) int {
	for i, value := range inputs {
		if i >= 25 {
			if !twoSum(inputs[i-25:i], value) {
				return value
			}
		}
	}
	return -1
}

// v This function from day 01
func twoSum(inputs []int, target int) bool {
	lookfor := make(map[int]bool, len(inputs))
	for _, v := range inputs {
		if lookfor[v] {
			return true
		}
		lookfor[target-v] = true
	}

	return false
}
