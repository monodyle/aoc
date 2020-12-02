package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.co/monodyle/aoc/helpers"
)

func main() {
	result := make(map[string]int, 2)
	err := helpers.ScanFile("input", func(s string) error {
		partOne, partTwo := validPassword(s)
		if partOne {
			result["partOne"]++
		}
		if partTwo {
			result["partTwo"]++
		}
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", result["partOne"])
	fmt.Println("Part One:", result["partTwo"])
}

func validPassword(s string) (bool, bool) {
	splitter := strings.Split(s, " ")
	r := rangeParser(splitter[0])
	c := rune(splitter[1][0])
	password := splitter[2]

	var count int = 0
	for _, v := range password {
		if v == c {
			count++
		}
	}

	partOne := r["min"] <= count && r["max"] >= count
	var letter byte = byte(c)
	partTwo := (password[r["min"]-1] == letter && password[r["max"]-1] != letter) || (password[r["min"]-1] != letter && password[r["max"]-1] == letter)

	return partOne, partTwo
}

func rangeParser(r string) map[string]int {
	splitter := strings.Split(r, "-")
	result := make(map[string]int, 2)

	min, _ := strconv.Atoi(splitter[0])
	result["min"] = min
	max, _ := strconv.Atoi(splitter[1])
	result["max"] = max

	return result
}
