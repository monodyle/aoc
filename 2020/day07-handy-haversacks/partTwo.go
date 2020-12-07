package main

func partTwo(container map[string][]BagCounter, bag string) int {
	var result int = 0
	for _, contain := range container[bag] {
		result += contain.amount + contain.amount*partTwo(container, contain.name)
	}
	return result
}
