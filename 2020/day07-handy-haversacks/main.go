package main

import (
	"fmt"
	"log"
	"regexp"
	"strconv"

	"github.co/monodyle/aoc/helpers"
)

const myBag = "shiny gold"

var (
	reRule        = regexp.MustCompile(`^(\w+ \w+) bags contain (.+)\.$`)
	reContainRule = regexp.MustCompile(`(\d+) (\w+ \w+) bags?`)
)

// BagCounter struct
type BagCounter struct {
	name   string
	amount int
}

func main() {
	container, containedBy := make(map[string][]BagCounter), make(map[string]map[string]bool)

	err := helpers.ScanLines("./input", func(line string) error {
		matches := reRule.FindStringSubmatch(line)
		containingColor := matches[1]
		if matches[2] == "no other bags" {
			container[containingColor] = nil
		} else {
			containRule := reContainRule.FindAllStringSubmatch(matches[2], -1)
			for _, ms := range containRule {
				count, _ := strconv.Atoi(ms[1])
				color := ms[2]
				container[containingColor] = append(container[containingColor], BagCounter{
					name:   color,
					amount: count,
				})

				if containedBy[color] == nil {
					containedBy[color] = make(map[string]bool)
				}
				containedBy[color][containingColor] = true
			}
		}

		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part One:", partOne(containedBy, myBag))
	// fmt.Println("Part Two:", partTwo)
}

func slice(colorset map[string]bool) []string {
	colours := make([]string, len(colorset))
	var i int
	for color := range colorset {
		colours[i] = color
		i++
	}
	return colours
}
