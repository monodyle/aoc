package main

import (
	"fmt"
	"log"
	"regexp"
	"strings"

	"github.co/monodyle/aoc/2020/helpers"
)

func main() {
	var result []int = []int{0, 0}
	re := regexp.MustCompile(`(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):((?:#)?\w+)`)

	err := helpers.ScanGroups("./input", func(passports []string) error {
		passport := strings.Join(passports, "\n")
		matched := re.FindAllStringSubmatch(passport, -1)
		if len(matched) == 7 || len(matched) == 8 {
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
		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", result[0])
	fmt.Println("Part Two:", result[1])
}
