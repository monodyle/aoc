package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"

	"github.co/monodyle/aoc/helpers"
)

// Range struct for bounds
type Range struct {
	min int
	max int
}

func main() {
	result := []int{0, 0}
	err := helpers.ScanFile("input", func(s string) error {
		partOne, partTwo := validPassword(s)
		if partOne {
			result[0]++
		}
		if partTwo {
			result[1]++
		}
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", result[0])
	fmt.Println("Part One:", result[1])
}

func validPassword(s string) (bool, bool) {
	splitter := strings.Split(s, " ")
	var r Range = rangeParser(splitter[0])
	c := rune(splitter[1][0])
	password := splitter[2]

	var count int = 0
	for _, v := range password {
		if v == c {
			count++
		}
	}

	partOne := r.min <= count && r.max >= count
	var letter byte = byte(c)
	partTwo := (password[r.min-1] == letter && password[r.max-1] != letter) || (password[r.min-1] != letter && password[r.max-1] == letter)

	return partOne, partTwo
}

func rangeParser(r string) Range {
	splitter := strings.Split(r, "-")

	min, _ := strconv.Atoi(splitter[0])
	max, _ := strconv.Atoi(splitter[1])

	return Range{min, max}
}
