package main

func partOne(containedBy map[string]map[string]bool, bag string) int {
	checkers := slice(containedBy[bag])
	passed := make(map[string]bool)
	for len(checkers) > 0 {
		var c string
		c, checkers = checkers[0], checkers[1:]
		if _, ok := passed[c]; ok {
			continue
		}
		passed[c] = true

		checkers = append(checkers, slice(containedBy[c])...)
	}

	return len(passed)
}
