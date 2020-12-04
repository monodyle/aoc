package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strings"
)

func main() {
	var result []int = []int{0, 0}
	content, err := ioutil.ReadFile("./input")
	if err != nil {
		log.Fatal(err)
	}
	passports := strings.Split(strings.TrimSpace(string(content)), "\n\n")
	re := regexp.MustCompile(`(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):((?:#)?\w+)`)

	for _, passport := range passports {
		matched := re.FindAllStringSubmatch(passport, -1)
		if len(matched) < 7 || len(matched) > 8 {
			continue
		}

		var checker map[string]string = make(map[string]string, 8)
		for _, field := range matched {
			checker[field[1]] = field[2]
		}
		if partOne(checker) {
			result[0]++
			if partTwo(checker) {
				result[1]++
			}
		}
	}

	fmt.Println("Part One:", result[0])
	fmt.Println("Part Two:", result[1])
}
