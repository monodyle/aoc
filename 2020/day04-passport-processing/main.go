package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("./input")
	if err != nil {
		log.Fatal(err)
	}
	passports := strings.Split(strings.TrimSpace(string(content)), "\n\n")

	fmt.Println("Part One:", partOne(passports))
}

func partOne(passports []string) int {
	var valids = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	var count int = 0
	for _, p := range passports {
		if contains(p, valids) {
			count++
		}
	}
	return count
}

func contains(input string, words []string) bool {
	var matches int = 0
	for _, word := range words {
		if strings.Contains(input, word) {
			matches++
		}
	}
	return matches == len(words)
}
