package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	var result []int = []int{0, 0}
	content, err := ioutil.ReadFile("./input")
	if err != nil {
		log.Fatal(err)
	}
	groups := strings.Split(strings.TrimSpace(string(content)), "\n\n")

	for _, group := range groups {
		result[0] += partOne(group)
		result[1] += partTwo(group)
	}

	fmt.Println("Part One:", result[0])
	fmt.Println("Part Two:", result[1])
}

func partOne(group string) int {
	var result int = 0
	var ans string = ""
	for _, c := range strings.Replace(group, "\n", "", -1) {
		if !strings.Contains(ans, string(c)) {
			ans += string(c)
		}
	}
	result += len(ans)
	return result
}

func partTwo(group string) int {
	var result int = 0
	var ans string = ""

	var decs = strings.Split(group, "\n")
	for _, c := range strings.Replace(group, "\n", "", -1) {
		if !strings.Contains(ans, string(c)) {
			ans += string(c)
		}
	}

	for _, c := range ans {
		var count int = 0
		for _, d := range decs {
			if strings.ContainsRune(d, c) {
				count++
			}
		}
		if count == len(decs) {
			result++
		}
	}

	return result
}
